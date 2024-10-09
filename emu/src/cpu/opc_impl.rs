#![allow(non_snake_case)]

use crate::bits;
use crate::cmn::LoHi;
use crate::cpu::{
    am,
    cmn::{IRQ_VECTOR, STACK_POINTER_HI},
    core::*,
};
use crate::riot::Memory;

pub mod load_store {
    use super::*;

    #[inline]
    pub fn reg_X(cpu: &NMOS6502) -> u8 {
        cpu.x()
    }

    #[inline]
    pub fn set_reg_X(cpu: &mut NMOS6502, val: u8) {
        cpu.set_x(val);

        psr_utils::sync_pcr_n(cpu, val);
        psr_utils::sync_pcr_z(cpu, val);
    }

    #[inline]
    pub fn A_AND_X(cpu: &NMOS6502) -> u8 {
        cpu.a() & cpu.x()
    }

    #[inline]
    pub fn reg_Y(cpu: &NMOS6502) -> u8 {
        cpu.y()
    }

    #[inline]
    pub fn set_reg_Y(cpu: &mut NMOS6502, val: u8) {
        cpu.set_y(val);

        psr_utils::sync_pcr_n(cpu, val);
        psr_utils::sync_pcr_z(cpu, val);
    }

    #[inline]
    pub fn reg_A(cpu: &NMOS6502) -> u8 {
        cpu.a()
    }

    #[inline]
    pub fn set_reg_A(cpu: &mut NMOS6502, val: u8) {
        cpu.set_a(val);

        psr_utils::sync_pcr_n(cpu, val);
        psr_utils::sync_pcr_z(cpu, val);
    }

    #[inline]
    pub fn set_regs_AX(cpu: &mut NMOS6502, val: u8) {
        cpu.set_a(val);
        cpu.set_x(val);

        psr_utils::sync_pcr_n(cpu, val);
        psr_utils::sync_pcr_z(cpu, val);
    }
}

pub mod transfer {
    use super::*;

    #[inline]
    pub fn TXA_core(cpu: &mut NMOS6502) {
        let x = cpu.x();
        cpu.set_a(x);

        psr_utils::sync_pcr_n(cpu, x);
        psr_utils::sync_pcr_z(cpu, x);
    }

    #[inline]
    pub fn TYA_core(cpu: &mut NMOS6502) {
        let y = cpu.y();
        cpu.set_a(y);

        psr_utils::sync_pcr_n(cpu, y);
        psr_utils::sync_pcr_z(cpu, y);
    }

    #[inline]
    pub fn TXS_core(cpu: &mut NMOS6502) {
        let x = cpu.x();
        cpu.set_s(x);
    }

    #[inline]
    pub fn TAY_core(cpu: &mut NMOS6502) {
        let a = cpu.a();
        cpu.set_y(a);

        psr_utils::sync_pcr_n(cpu, a);
        psr_utils::sync_pcr_z(cpu, a);
    }

    #[inline]
    pub fn TAX_core(cpu: &mut NMOS6502) {
        let a = cpu.a();
        cpu.set_x(a);

        psr_utils::sync_pcr_n(cpu, a);
        psr_utils::sync_pcr_z(cpu, a);
    }

    #[inline]
    pub fn TSX_core(cpu: &mut NMOS6502) {
        let s = cpu.s();
        cpu.set_x(s);

        psr_utils::sync_pcr_n(cpu, s);
        psr_utils::sync_pcr_z(cpu, s);
    }
}

pub mod stack {
    use super::*;

    /// NOTE: Flags B & __ will be inserted when PSR is transferred to the stack by software (BRK or PHP).
    #[inline]
    pub fn reg_PSR(cpu: &NMOS6502) -> u8 {
        cpu.psr() | 0x30
    }

    /// NOTE: Flags B & __ are ignored when retrieved by software (PLP or RTI).
    #[inline]
    pub fn set_reg_PSR(cpu: &mut NMOS6502, val: u8) {
        cpu.set_psr(val & !0x30);
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use test_case::test_case;

        #[test_case(0b0000_0000)]
        #[test_case(0b1010_0010)]
        #[test_case(0b0101_1001)]
        #[test_case(0b1111_1111)]
        fn push_psr_always_keeps_bits_4_and_5_on(psr: u8) {
            let mut cpu = NMOS6502::default();

            cpu.set_psr(psr);

            let stack_psr = reg_PSR(&cpu);

            assert_eq!(
                stack_psr & 0b1100_1111,
                psr & 0b1100_1111,
                "all bits other 4 & 5 should be on stack."
            );
            assert!(
                bits::tst_bits(stack_psr, 0x30),
                "bits 4 & 5 should always be on stack."
            );
        }

        #[test_case(0b1111_1111)]
        #[test_case(0b1110_0000)]
        #[test_case(0b1101_0100)]
        #[test_case(0b0100_0011)]
        fn pop_psr_always_keep_bits_4_and_5_off(psr: u8) {
            let mut cpu = NMOS6502::default();

            set_reg_PSR(&mut cpu, psr);

            assert!(
                bits::tst_bits(cpu.psr() & 0b0011_0000, 0b0000_0000),
                "bits 4 & 5 should always be 0."
            );
            assert!(
                bits::tst_bits(cpu.psr(), psr & 0b1100_1111),
                "except bits 4 & 5 psr after pop should match."
            );
        }
    }
}

pub mod shift {
    use super::*;

    #[inline]
    pub fn ASL_A(cpu: &mut NMOS6502) {
        let old_v = cpu.a();

        let new_v = ASL_core(cpu, old_v);

        cpu.set_a(new_v);
    }

    #[inline]
    pub fn ASL_core(cpu: &mut NMOS6502, old_v: u8) -> u8 {
        let new_v = old_v << 1;
        psr_utils::sync_pcr_n(cpu, new_v);
        psr_utils::sync_pcr_z(cpu, new_v);
        psr_utils::shift_ops_sync_pcr_c_msb(cpu, old_v);

        new_v
    }

    #[inline]
    pub fn ROL_A(cpu: &mut NMOS6502) {
        let old_v = cpu.a();
        let new_v = ROL_core(cpu, old_v);
        cpu.set_a(new_v);
    }

    #[inline]
    pub fn ROL_core(cpu: &mut NMOS6502, old_v: u8) -> u8 {
        let new_v = ROL_flags(cpu, old_v);
        psr_utils::sync_pcr_n(cpu, new_v);
        psr_utils::sync_pcr_z(cpu, new_v);
        psr_utils::shift_ops_sync_pcr_c_msb(cpu, old_v);

        new_v
    }

    pub fn LSR_A(cpu: &mut NMOS6502) {
        let old_v = cpu.a();
        let new_v = LSR_core(cpu, old_v);
        cpu.set_a(new_v);
    }

    #[inline]
    pub fn LSR_core(cpu: &mut NMOS6502, old_v: u8) -> u8 {
        let new_v = old_v >> 1;
        cpu.clr_psr_bit(PSR::N);
        psr_utils::sync_pcr_z(cpu, new_v);
        psr_utils::shift_ops_sync_pcr_c_lsb(cpu, old_v);

        new_v
    }

    pub fn ROR_A(cpu: &mut NMOS6502) {
        let old_v = cpu.a();
        let new_v = ROR_core(cpu, old_v);
        cpu.set_a(new_v);
    }

    #[inline]
    pub fn ROR_core(cpu: &mut NMOS6502, old_v: u8) -> u8 {
        let new_v = ROR_flags(cpu, old_v);
        psr_utils::sync_pcr_n(cpu, new_v);
        psr_utils::sync_pcr_z(cpu, new_v);
        psr_utils::shift_ops_sync_pcr_c_lsb(cpu, old_v);

        new_v
    }

    #[inline]
    pub fn ROR_flags(cpu: &NMOS6502, val: u8) -> u8 {
        (val >> 1)
            | if cpu.tst_psr_bit(PSR::C) {
                0b10000000
            } else {
                0b00000000
            }
    }

    #[inline]
    pub fn ROL_flags(cpu: &NMOS6502, val: u8) -> u8 {
        (val << 1)
            | if cpu.tst_psr_bit(PSR::C) {
                0b00000001
            } else {
                0b00000000
            }
    }
}

pub mod logic {
    use super::*;

    #[inline]
    pub fn AND_core(cpu: &mut NMOS6502, val: u8) {
        let res = cpu.a() & val;
        cpu.set_a(res);

        psr_utils::sync_pcr_n(cpu, res);
        psr_utils::sync_pcr_z(cpu, res);
    }

    #[inline]
    pub fn EOR_core(cpu: &mut NMOS6502, val: u8) {
        let res = cpu.a() ^ val;
        cpu.set_a(res);

        psr_utils::sync_pcr_n(cpu, res);
        psr_utils::sync_pcr_z(cpu, res);
    }

    #[inline]
    pub fn ORA_core(cpu: &mut NMOS6502, val: u8) {
        let res = cpu.a() | val;
        cpu.set_a(res);

        psr_utils::sync_pcr_n(cpu, res);
        psr_utils::sync_pcr_z(cpu, res);
    }

    #[inline]
    pub fn BIT_core(cpu: &mut NMOS6502, v2: u8) {
        let v1 = cpu.a();
        let res = v1 & v2;

        psr_utils::sync_pcr_n(cpu, v2);
        if bits::tst_bits(v2, 0b0100_0000) {
            cpu.set_psr_bit(PSR::V)
        } else {
            cpu.clr_psr_bit(PSR::V)
        }
        psr_utils::sync_pcr_z(cpu, res);
    }
}

pub mod arithmetic {
    use super::*;

    #[inline]
    pub fn CMP_X_core(cpu: &mut NMOS6502, val: u8) {
        CMP_core(cpu, cpu.x(), val);
    }

    #[inline]
    pub fn CMP_Y_core(cpu: &mut NMOS6502, val: u8) {
        CMP_core(cpu, cpu.y(), val);
    }

    pub fn CPY_core(cpu: &mut NMOS6502, val: u8) {
        CMP_core(cpu, cpu.y(), val);
    }

    pub fn CPX_core(cpu: &mut NMOS6502, val: u8) {
        CMP_core(cpu, cpu.x(), val);
    }

    #[inline]
    pub fn CMP_A_core(cpu: &mut NMOS6502, val: u8) {
        CMP_core(cpu, cpu.a(), val);
    }

    #[inline]
    pub fn CMP_core(cpu: &mut NMOS6502, n1: u8, n2: u8) {
        let res = safe_SUB_checked(n1, n2);
        psr_utils::sync_pcr_n(cpu, res.0);
        psr_utils::sync_pcr_z(cpu, res.0);
        if n1 < n2 {
            cpu.clr_psr_bit(PSR::C);
        } else {
            cpu.set_psr_bit(PSR::C);
        }
    }

    #[inline]
    pub fn DCP_core(cpu: &mut NMOS6502, val: u8) -> u8 {
        let val = val.wrapping_sub(1);

        CMP_A_core(cpu, val);

        val
    }

    #[inline]
    pub fn ISC_core(cpu: &mut NMOS6502, val: u8) -> u8 {
        let val = val.wrapping_add(1);

        SBC_core(cpu, val);

        val
    }

    #[inline]
    pub fn safe_SUB_checked(val1: u8, val2: u8) -> (u8, bool) {
        let res = val1 as i16 - val2 as i16;

        let v = res & 0b1_0000_0000 != 0;

        (res as u8, v)
    }

    /// Refer:
    /// - https://www.masswerk.at/6502/6502_instruction_set.html#arithmetic
    /// - https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    /// - http://www.6502.org/tutorials/decimal_mode.html
    #[inline]
    pub fn ADC_core(cpu: &mut NMOS6502, n2: u8) {
        if cpu.tst_psr_bit(PSR::D) {
            ADC_core_bcd(cpu, n2)
        } else {
            ADC_core_bin(cpu, n2)
        }
    }

    pub fn ADC_core_bin(cpu: &mut NMOS6502, n2: u8) {
        let n1 = cpu.a();
        let res = n1 as u16 + n2 as u16 + if cpu.tst_psr_bit(PSR::C) { 0x01 } else { 0x00 };
        let res_u8 = res as u8;
        cpu.set_a(res_u8);

        psr_utils::sync_pcr_n(cpu, res_u8);
        let bit8u8 = 0b1000_0000;
        let c6 = ((n1 & !bit8u8) + (n2 & !bit8u8)) & bit8u8 == bit8u8;
        let bit8u16 = 0b0000_0001_0000_0000;
        let c7 = res & bit8u16 == bit8u16;
        if c6 != c7 {
            cpu.set_psr_bit(PSR::V)
        } else {
            cpu.clr_psr_bit(PSR::V)
        }
        psr_utils::sync_pcr_z(cpu, res_u8);
        if c7 {
            cpu.set_psr_bit(PSR::C)
        } else {
            cpu.clr_psr_bit(PSR::C)
        }
    }

    pub fn ADC_core_bcd(_cpu: &mut NMOS6502, _n2: u8) {
        todo!("ADC in decimal mode is not yet implemented.")
    }

    /// Refer:
    /// - http://forum.6502.org/viewtopic.php?f=2&t=2944#p57780
    #[inline]
    pub fn SBC_core(cpu: &mut NMOS6502, n2: u8) {
        if cpu.tst_psr_bit(PSR::D) {
            SBC_core_bcd(cpu, n2)
        } else {
            SBC_core_bin(cpu, n2)
        }
    }

    pub fn SBC_core_bin(cpu: &mut NMOS6502, n2: u8) {
        ADC_core(cpu, !n2);
    }

    pub fn SBC_core_bcd(_cpu: &mut NMOS6502, _n2: u8) {
        todo!("SBC in decimal mode is not yet implemented.")
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use test_case::test_case;

        #[test_case(0x10, 0x10, (0x00, false))]
        #[test_case(0x00, 0x01, (0xFF, true))]
        #[test_case(0x10, 0x20, (0xF0, true))]
        fn test_safe_sub(v1: u8, v2: u8, exp: (u8, bool)) {
            let obt = safe_SUB_checked(v1, v2);
            assert_eq!(exp, obt);
        }

        ///           C     n1    n2    res    N      V      Z      C
        #[test_case(false, 0x00, 0x00, 0x00, false, false, true, false)]
        #[test_case(true, 0x01, 0x01, 0x03, false, false, false, false)]
        #[test_case(false, 0x01, 0x02, 0x03, false, false, false, false)]
        #[test_case(false, 0x64, 0xE8, 0x4C, false, false, false, true)]
        #[test_case(false, 0x40, 0x80, 0xC0, true, false, false, false)]
        #[test_case(true, 0xD0, 0x8F, 0x60, false, true, false, true)]
        // Test cases from https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        #[test_case(false, 0x50, 0x10, 0x60, false, false, false, false)]
        #[test_case(false, 0x50, 0x50, 0xA0, true, true, false, false)]
        #[test_case(false, 0x50, 0x90, 0xE0, true, false, false, false)]
        #[test_case(false, 0x50, 0xD0, 0x20, false, false, false, true)]
        #[test_case(false, 0xD0, 0x10, 0xE0, true, false, false, false)]
        #[test_case(false, 0xD0, 0x50, 0x20, false, false, false, true)]
        #[test_case(false, 0xD0, 0x90, 0x60, false, true, false, true)]
        #[test_case(false, 0xD0, 0xD0, 0xA0, true, false, false, true)]
        #[allow(clippy::too_many_arguments)]
        fn test_binary_adc(
            carry: bool,
            v1: u8,
            v2: u8,
            exp: u8,
            exp_n: bool,
            exp_v: bool,
            exp_z: bool,
            exp_c: bool,
        ) {
            let mut cpu = NMOS6502::default();
            cpu.clr_psr_bit(PSR::D);
            if carry {
                cpu.set_psr_bit(PSR::C)
            } else {
                cpu.clr_psr_bit(PSR::C)
            }
            cpu.set_a(v1);

            ADC_core(&mut cpu, v2);

            assert_eq!(cpu.a(), exp);
            assert_eq!(cpu.tst_psr_bit(PSR::N), exp_n, "N flag mismatch");
            assert_eq!(cpu.tst_psr_bit(PSR::V), exp_v, "V flag mismatch");
            assert_eq!(cpu.tst_psr_bit(PSR::Z), exp_z, "Z flag mismatch");
            assert_eq!(cpu.tst_psr_bit(PSR::C), exp_c, "C flag mismatch");
        }

        ///           C     n1    n2    res    N      V      Z      C
        #[test_case(true, 0x00, 0x00, 0x00, false, true, true, true)]
        // Test cases from https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        #[test_case(true, 0x50, 0xF0, 0x60, false, false, false, false)]
        #[test_case(true, 0x50, 0xB0, 0xA0, true, true, false, false)]
        #[test_case(true, 0x50, 0x70, 0xE0, true, false, false, false)]
        #[test_case(true, 0x50, 0x30, 0x20, false, false, false, true)]
        #[test_case(true, 0xD0, 0xF0, 0xE0, true, false, false, false)]
        #[test_case(true, 0xD0, 0xB0, 0x20, false, false, false, true)]
        #[test_case(true, 0xD0, 0x70, 0x60, false, true, false, true)]
        #[test_case(true, 0xD0, 0x30, 0xA0, true, false, false, true)]
        #[allow(clippy::too_many_arguments)]
        fn test_binary_sbc(
            carry: bool,
            v1: u8,
            v2: u8,
            exp: u8,
            exp_n: bool,
            exp_v: bool,
            exp_z: bool,
            exp_c: bool,
        ) {
            let mut cpu = NMOS6502::default();
            cpu.clr_psr_bit(PSR::D);
            if carry {
                cpu.set_psr_bit(PSR::C)
            } else {
                cpu.clr_psr_bit(PSR::C)
            }
            cpu.set_a(v1);

            SBC_core(&mut cpu, v2);

            assert_eq!(cpu.a(), exp);
            assert_eq!(cpu.tst_psr_bit(PSR::N), exp_n, "N flag mismatch");
            assert_eq!(cpu.tst_psr_bit(PSR::V), exp_v, "V flag mismatch");
            assert_eq!(cpu.tst_psr_bit(PSR::Z), exp_z, "Z flag mismatch");
            assert_eq!(cpu.tst_psr_bit(PSR::C), exp_c, "C flag mismatch");
        }
    }
}

pub mod arithmetic_inc_dec {
    use super::*;

    #[inline]
    pub fn DEY_core(cpu: &mut NMOS6502) {
        let val = DEC_core(cpu, cpu.y());
        cpu.set_y(val);
    }

    #[inline]
    pub fn INY_core(cpu: &mut NMOS6502) {
        let val = INC_core(cpu, cpu.y());
        cpu.set_y(val);
    }

    #[inline]
    pub fn DEX_core(cpu: &mut NMOS6502) {
        let val = DEC_core(cpu, cpu.x());
        cpu.set_x(val);
    }

    #[inline]
    pub fn DEC_core(cpu: &mut NMOS6502, val: u8) -> u8 {
        let val = val.wrapping_sub(1);

        psr_utils::sync_pcr_n(cpu, val);
        psr_utils::sync_pcr_z(cpu, val);

        val
    }

    #[inline]
    pub fn INC_core(cpu: &mut NMOS6502, val: u8) -> u8 {
        let val = val.wrapping_add(1);

        psr_utils::sync_pcr_n(cpu, val);
        psr_utils::sync_pcr_z(cpu, val);

        val
    }

    #[inline]
    pub fn INX_core(cpu: &mut NMOS6502) {
        let val = INC_core(cpu, cpu.x());
        cpu.set_x(val);
    }
}

pub mod control_flow_branch {
    use super::*;

    #[inline]
    pub fn BPL_core(cpu: &NMOS6502) -> bool {
        !cpu.tst_psr_bit(PSR::N)
    }

    #[inline]
    pub fn BMI_core(cpu: &NMOS6502) -> bool {
        cpu.tst_psr_bit(PSR::N)
    }

    #[inline]
    pub fn BVC_core(cpu: &NMOS6502) -> bool {
        !cpu.tst_psr_bit(PSR::V)
    }

    #[inline]
    pub fn BVS_core(cpu: &NMOS6502) -> bool {
        cpu.tst_psr_bit(PSR::V)
    }

    #[inline]
    pub fn BCC_core(cpu: &NMOS6502) -> bool {
        !cpu.tst_psr_bit(PSR::C)
    }

    #[inline]
    pub fn BCS_core(cpu: &NMOS6502) -> bool {
        cpu.tst_psr_bit(PSR::C)
    }

    #[inline]
    pub fn BNE_core(cpu: &NMOS6502) -> bool {
        !cpu.tst_psr_bit(PSR::Z)
    }

    #[inline]
    pub fn BEQ_core(cpu: &NMOS6502) -> bool {
        cpu.tst_psr_bit(PSR::Z)
    }
}

pub mod flags {
    use super::*;

    #[inline]
    pub fn CLC_core(cpu: &mut NMOS6502) {
        cpu.clr_psr_bit(PSR::C);
    }
    #[inline]
    pub fn SEC_core(cpu: &mut NMOS6502) {
        cpu.set_psr_bit(PSR::C);
    }

    #[inline]
    pub fn CLI_core(cpu: &mut NMOS6502) {
        cpu.clr_psr_bit(PSR::I);
    }

    #[inline]
    pub fn SEI_core(cpu: &mut NMOS6502) {
        cpu.set_psr_bit(PSR::I);
    }

    #[inline]
    pub fn CLV_core(cpu: &mut NMOS6502) {
        cpu.clr_psr_bit(PSR::V);
    }

    #[inline]
    pub fn CLD_core(cpu: &mut NMOS6502) {
        cpu.clr_psr_bit(PSR::D);
    }

    #[inline]
    pub fn SED_core(cpu: &mut NMOS6502) {
        cpu.set_psr_bit(PSR::D);
    }
}

pub mod nop {
    use super::*;

    #[inline]
    pub fn NOP_1(_: &mut NMOS6502) {}

    #[inline]
    pub fn NOP_2(_: &mut NMOS6502, _: u8) {}
}

mod psr_utils {
    use super::*;

    #[inline]
    fn shift_ops_sync_pcr_c(cpu: &mut NMOS6502, val: u8, bit_selector: u8) {
        if bits::tst_bits(val, bit_selector) {
            cpu.set_psr_bit(PSR::C)
        } else {
            cpu.clr_psr_bit(PSR::C)
        }
    }

    #[inline]
    pub fn shift_ops_sync_pcr_c_lsb(cpu: &mut NMOS6502, val: u8) {
        shift_ops_sync_pcr_c(cpu, val, 0b0000_0001);
    }

    #[inline]
    pub fn shift_ops_sync_pcr_c_msb(cpu: &mut NMOS6502, val: u8) {
        shift_ops_sync_pcr_c(cpu, val, 0b1000_0000);
    }

    #[inline]
    pub fn sync_pcr_z(cpu: &mut NMOS6502, val: u8) {
        if val == 0 {
            cpu.set_psr_bit(PSR::Z)
        } else {
            cpu.clr_psr_bit(PSR::Z)
        }
    }

    #[inline]
    pub fn sync_pcr_n(cpu: &mut NMOS6502, val: u8) {
        if bits::tst_bits(val, 0b1000_0000) {
            cpu.set_psr_bit(PSR::N)
        } else {
            cpu.clr_psr_bit(PSR::N)
        }
    }
}

/// Refer: https://www.nesdev.org/6502_cpu.txt
#[rustfmt::skip]
pub const ALL_OPCODE_STEPS: &[OpCodeSteps; 0x1_00] = &[
    /* 0x00 - impl | BRK */
    am::stack::opcode_steps_BRK!(stack::reg_PSR, am::opc_step_illegal),
    /* 0x01 - (ind,X) | ORA (oper,X) */
    am::pre_indexed_indirect::opcode_steps_read!(logic::ORA_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x02 */ am::stub_opcode_steps!(),
    /* 0x03 */ am::stub_opcode_steps!(),
    /* 0x04 - zpg | NOP oper */
    am::zero_page::opcode_steps_read!(nop::NOP_2, am::opc_step_illegal),
    /* 0x05 - zpg | ORA oper */
    am::zero_page::opcode_steps_read!(logic::ORA_core, am::opc_step_illegal),
    /* 0x06 - zpg | ASL oper */
    am::zero_page::opcode_steps_read_modify_write!(shift::ASL_core, am::opc_step_illegal),
    /* 0x07 */ am::stub_opcode_steps!(),
    /* 0x08 - impl | PHP */
    am::stack::opcode_steps_PHX!(stack::reg_PSR, am::opc_step_illegal),
    /* 0x09 - # | ORA #oper */
    am::immediate::opcode_steps!(logic::ORA_core, am::opc_step_illegal),
    /* 0x0A - A | ASL A */
    am::implied::opcode_steps!(shift::ASL_A, am::opc_step_illegal),
    /* 0x0B */ am::stub_opcode_steps!(),
    /* 0x0C - abs | NOP oper */
    am::absolute::opcode_steps_read!(nop::NOP_2, am::opc_step_illegal),
    /* 0x0D - abs | ORA oper */
    am::absolute::opcode_steps_read!(logic::ORA_core, am::opc_step_illegal),
    /* 0x0E - abs | ASL oper */
    am::absolute::opcode_steps_read_modify_write!(shift::ASL_core, am::opc_step_illegal),
    /* 0x0F */ am::stub_opcode_steps!(),
    /* 0x10 - rel | BPL oper */
    am::relative::opcode_steps!(control_flow_branch::BPL_core, am::opc_step_illegal),
    /* 0x11 - (ind),Y | ORA (oper),Y */
    am::post_indexed_indirect::opcode_steps_read!(logic::ORA_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0x12 */ am::stub_opcode_steps!(),
    /* 0x13 */ am::stub_opcode_steps!(),
    /* 0x14 - zpg,X | NOP oper,X */
    am::indexed_zero_page::opcode_steps_read!(nop::NOP_2, load_store::reg_X, am::opc_step_illegal),
    /* 0x15 - zpg,X | ORA oper,X */
    am::indexed_zero_page::opcode_steps_read!(logic::ORA_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x16 - zpg,X | ASL oper,X */
    am::indexed_zero_page::opcode_steps_read_modify_write!(shift::ASL_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x17 */ am::stub_opcode_steps!(),
    /* 0x18 -  impl | CLC */
    am::implied::opcode_steps!(flags::CLC_core, am::opc_step_illegal),
    /* 0x19 - abs,Y | ORA oper,Y */
    am::indexed_absolute::opcode_steps_read!(logic::ORA_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0x1A - impl | NOP */
    am::implied::opcode_steps!(nop::NOP_1, am::opc_step_illegal),
    /* 0x1B */ am::stub_opcode_steps!(),
    /* 0x1C - abs,X | NOP oper,X */
    am::indexed_absolute::opcode_steps_read!(nop::NOP_2, load_store::reg_X, am::opc_step_illegal),
    /* 0x1D - abs,X | ORA oper,X */
    am::indexed_absolute::opcode_steps_read!(logic::ORA_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x1E - abs,X | ASL oper,X */
    am::indexed_absolute::opcode_steps_read_modify_write!(shift::ASL_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x1F */ am::stub_opcode_steps!(),
    /* 0x20 - abs | JSR oper */
    am::stack::opcode_steps_JSR!(am::opc_step_illegal),
    /* 0x21 - (ind,X) | AND (oper,X) */
    am::pre_indexed_indirect::opcode_steps_read!(logic::AND_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x22 */ am::stub_opcode_steps!(),
    /* 0x23 */ am::stub_opcode_steps!(),
    /* 0x24 - zpg | BIT oper */
    am::zero_page::opcode_steps_read!(logic::BIT_core, am::opc_step_illegal),
    /* 0x25 - zpg | AND oper */
    am::zero_page::opcode_steps_read!(logic::AND_core, am::opc_step_illegal),
    /* 0x26 - zpg | ROL oper */
    am::zero_page::opcode_steps_read_modify_write!(shift::ROL_core, am::opc_step_illegal),
    /* 0x27 */ am::stub_opcode_steps!(),
    /* 0x28 - impl | PLP */
    am::stack::opcode_steps_PLX!(stack::set_reg_PSR, am::opc_step_illegal),
    /* 0x29 - # | AND #oper */
    am::immediate::opcode_steps!(logic::AND_core, am::opc_step_illegal),
    /* 0x2A - A | ROL A */
    am::implied::opcode_steps!(shift::ROL_A, am::opc_step_illegal),
    /* 0x2B */ am::stub_opcode_steps!(),
    /* 0x2C - abs | BIT oper */
    am::absolute::opcode_steps_read!(logic::BIT_core, am::opc_step_illegal),
    /* 0x2D - abs | AND oper */
    am::absolute::opcode_steps_read!(logic::AND_core, am::opc_step_illegal),
    /* 0x2E - abs | ROL oper */
    am::absolute::opcode_steps_read_modify_write!(shift::ROL_core, am::opc_step_illegal),
    /* 0x2F */ am::stub_opcode_steps!(),
    /* 0x30 - rel | BMI oper */
    am::relative::opcode_steps!(control_flow_branch::BMI_core, am::opc_step_illegal),
    /* 0x31 - (ind),Y | AND (oper),Y */
    am::post_indexed_indirect::opcode_steps_read!(logic::AND_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0x32 */ am::stub_opcode_steps!(),
    /* 0x33 */ am::stub_opcode_steps!(),
    /* 0x34 - zpg,X | NOP oper,X */
    am::indexed_zero_page::opcode_steps_read!(nop::NOP_2, load_store::reg_X, am::opc_step_illegal),
    /* 0x35 - zpg,X | AND oper,X */
    am::indexed_zero_page::opcode_steps_read!(logic::AND_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x36 - zpg,X | ROL oper,X */
    am::indexed_zero_page::opcode_steps_read_modify_write!(shift::ROL_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x37 */ am::stub_opcode_steps!(),
    /* 0x38 - impl | SEC */
    am::implied::opcode_steps!(flags::SEC_core, am::opc_step_illegal),
    /* 0x39 - abs,Y | AND oper,Y */
    am::indexed_absolute::opcode_steps_read!(logic::AND_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0x3A - impl | NOP */
    am::implied::opcode_steps!(nop::NOP_1, am::opc_step_illegal),
    /* 0x3B */ am::stub_opcode_steps!(),
    /* 0x3C - abs,X | NOP oper,X */
    am::indexed_absolute::opcode_steps_read!(nop::NOP_2, load_store::reg_X, am::opc_step_illegal),
    /* 0x3D - abs,X | AND oper,X */
    am::indexed_absolute::opcode_steps_read!(logic::AND_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x3E - abs,X | ROL oper,X */
    am::indexed_absolute::opcode_steps_read_modify_write!(shift::ROL_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x3F */ am::stub_opcode_steps!(),
    /* 0x40 - impl | RTI */
    am::stack::opcode_steps_RTI!(stack::set_reg_PSR, am::opc_step_illegal),
    /* 0x41 - (ind,X) | EOR (oper,X) */
    am::pre_indexed_indirect::opcode_steps_read!(logic::EOR_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x42 */ am::stub_opcode_steps!(),
    /* 0x43 */ am::stub_opcode_steps!(),
    /* 0x44 - zpg | NOP oper */
    am::zero_page::opcode_steps_read!(nop::NOP_2, am::opc_step_illegal),
    /* 0x45 - zpg | EOR oper */
    am::zero_page::opcode_steps_read!(logic::EOR_core, am::opc_step_illegal),
    /* 0x46 - zpg | LSR oper */
    am::zero_page::opcode_steps_read_modify_write!(shift::LSR_core, am::opc_step_illegal),
    /* 0x47 */ am::stub_opcode_steps!(),
    /* 0x48 - impl | PHA */
    am::stack::opcode_steps_PHX!(load_store::reg_A, am::opc_step_illegal),
    /* 0x49 - # | EOR #oper */
    am::immediate::opcode_steps!(logic::EOR_core, am::opc_step_illegal),
    /* 0x4A - A | LSR A */
    am::implied::opcode_steps!(shift::LSR_A, am::opc_step_illegal),
    /* 0x4B */ am::stub_opcode_steps!(),
    /* 0x4C - abs | JMP oper */
    am::absolute::opcode_steps_JMP!(am::opc_step_illegal),
    /* 0x4D - abs | EOR oper */
    am::absolute::opcode_steps_read!(logic::EOR_core, am::opc_step_illegal),
    /* 0x4E - abs | LSR oper */
    am::absolute::opcode_steps_read_modify_write!(shift::LSR_core, am::opc_step_illegal),
    /* 0x4F */ am::stub_opcode_steps!(),
    /* 0x50 - rel | BVC oper */
    am::relative::opcode_steps!(control_flow_branch::BVC_core, am::opc_step_illegal),
    /* 0x51 - (ind),Y | EOR (oper),Y */
    am::post_indexed_indirect::opcode_steps_read!(logic::EOR_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0x52 */ am::stub_opcode_steps!(),
    /* 0x53 */ am::stub_opcode_steps!(),
    /* 0x54 - zpg,X | NOP oper,X */
    am::indexed_zero_page::opcode_steps_read!(nop::NOP_2, load_store::reg_X, am::opc_step_illegal),
    /* 0x55 - zpg,X | EOR oper,X */
    am::indexed_zero_page::opcode_steps_read!(logic::EOR_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x56 - zpg,X | LSR oper,X */
    am::indexed_zero_page::opcode_steps_read_modify_write!(shift::LSR_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x57 */ am::stub_opcode_steps!(),
    /* 0x58 - impl | CLI */
    am::implied::opcode_steps!(flags::CLI_core, am::opc_step_illegal),
    /* 0x59 - abs,Y | EOR oper,Y */
    am::indexed_absolute::opcode_steps_read!(logic::EOR_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0x5A - impl | NOP */
    am::implied::opcode_steps!(nop::NOP_1, am::opc_step_illegal),
    /* 0x5B */ am::stub_opcode_steps!(),
    /* 0x5C - abs,X | NOP oper,X */
    am::indexed_absolute::opcode_steps_read!(nop::NOP_2, load_store::reg_X, am::opc_step_illegal),
    /* 0x5D - abs,X | EOR oper,X */
    am::indexed_absolute::opcode_steps_read!(logic::EOR_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x5E - abs,X | LSR oper,X */
    am::indexed_absolute::opcode_steps_read_modify_write!(shift::LSR_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x5F */ am::stub_opcode_steps!(),
    /* 0x60 - impl | RTS */
    am::stack::opcode_steps_RTS!(am::opc_step_illegal),
    /* 0x61 - (ind,X) | ADC (oper,X) */
    am::pre_indexed_indirect::opcode_steps_read!(arithmetic::ADC_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x62 */ am::stub_opcode_steps!(),
    /* 0x63 */ am::stub_opcode_steps!(),
    /* 0x64 - zpg | NOP oper */
    am::zero_page::opcode_steps_read!(nop::NOP_2, am::opc_step_illegal),
    /* 0x65 - zpg | ADC oper */
    am::zero_page::opcode_steps_read!(arithmetic::ADC_core, am::opc_step_illegal),
    /* 0x66 - zpg | ROR oper */
    am::zero_page::opcode_steps_read_modify_write!(shift::ROR_core, am::opc_step_illegal),
    /* 0x67 */ am::stub_opcode_steps!(),
    /* 0x68 - impl | PLA */
    am::stack::opcode_steps_PLX!(load_store::set_reg_A, am::opc_step_illegal),
    /* 0x69 - # | ADC #oper */
    am::immediate::opcode_steps!(arithmetic::ADC_core, am::opc_step_illegal),
    /* 0x6A - A | ROR A */
    am::implied::opcode_steps!(shift::ROR_A, am::opc_step_illegal),
    /* 0x6B */ am::stub_opcode_steps!(),
    /* 0x6C - ind | JMP (oper) */
    am::indirect::opcode_steps!(am::opc_step_illegal),
    /* 0x6D - abs | ADC oper */
    am::absolute::opcode_steps_read!(arithmetic::ADC_core, am::opc_step_illegal),
    /* 0x6E - abs | ROR oper */
    am::absolute::opcode_steps_read_modify_write!(shift::ROR_core, am::opc_step_illegal),
    /* 0x6F */ am::stub_opcode_steps!(),
    /* 0x70 - rel | BVS oper */
    am::relative::opcode_steps!(control_flow_branch::BVS_core, am::opc_step_illegal),
    /* 0x71 - (ind),Y | ADC (oper),Y */
    am::post_indexed_indirect::opcode_steps_read!(arithmetic::ADC_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0x72 */ am::stub_opcode_steps!(),
    /* 0x73 */ am::stub_opcode_steps!(),
    /* 0x74 - zpg,X | NOP oper,X */
    am::indexed_zero_page::opcode_steps_read!(nop::NOP_2, load_store::reg_X, am::opc_step_illegal),
    /* 0x75 - zpg,X | ADC oper,X */
    am::indexed_zero_page::opcode_steps_read!(arithmetic::ADC_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x76 - zpg,X | ROR oper,X */
    am::indexed_zero_page::opcode_steps_read_modify_write!(shift::ROR_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x77 */ am::stub_opcode_steps!(),
    /* 0x78 - impl | SEI */
    am::implied::opcode_steps!(flags::SEI_core, am::opc_step_illegal),
    /* 0x79 - abs,Y | ADC oper,Y */
    am::indexed_absolute::opcode_steps_read!(arithmetic::ADC_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0x7A - impl | NOP */
    am::implied::opcode_steps!(nop::NOP_1, am::opc_step_illegal),
    /* 0x7B */ am::stub_opcode_steps!(),
    /* 0x7C - abs,X | NOP oper,X */
    am::indexed_absolute::opcode_steps_read!(nop::NOP_2, load_store::reg_X, am::opc_step_illegal),
    /* 0x7D - abs,X | ADC oper,X */
    am::indexed_absolute::opcode_steps_read!(arithmetic::ADC_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x7E - abs,X | ROR oper,X */
    am::indexed_absolute::opcode_steps_read_modify_write!(shift::ROR_core, load_store::reg_X, am::opc_step_illegal),
    /* 0x7F */ am::stub_opcode_steps!(),
    /* 0x80 - # | NOP #oper */
    am::immediate::opcode_steps!(nop::NOP_2, am::opc_step_illegal),
    /* 0x81 - (ind,X) | STA (oper,X) */
    am::pre_indexed_indirect::opcode_steps_write!(load_store::reg_A, load_store::reg_X, am::opc_step_illegal),
    /* 0x82 - # | NOP #oper */
    am::immediate::opcode_steps!(nop::NOP_2, am::opc_step_illegal),
    /* 0x83 - (ind,X) | SAX (oper,X) */
    am::pre_indexed_indirect::opcode_steps_write!(load_store::A_AND_X, load_store::reg_X, am::opc_step_illegal),
    /* 0x84 - zpg | STY oper */
    am::zero_page::opcode_steps_write!(load_store::reg_Y, am::opc_step_illegal),
    /* 0x85 - zpg | STA oper */
    am::zero_page::opcode_steps_write!(load_store::reg_A, am::opc_step_illegal),
    /* 0x86 - zpg | STX oper */
    am::zero_page::opcode_steps_write!(load_store::reg_X, am::opc_step_illegal),
    /* 0x87 - zpg | SAX oper */
    am::zero_page::opcode_steps_write!(load_store::A_AND_X, am::opc_step_illegal),
    /* 0x88 - impl | DEY */
    am::implied::opcode_steps!(arithmetic_inc_dec::DEY_core, am::opc_step_illegal),
    /* 0x89 - # | NOP #oper */
    am::immediate::opcode_steps!(nop::NOP_2, am::opc_step_illegal),
    /* 0x8A - impl | TXA */
    am::implied::opcode_steps!(transfer::TXA_core, am::opc_step_illegal),
    /* 0x8B */ am::stub_opcode_steps!(),
    /* 0x8C - abs | STY oper */
    am::absolute::opcode_steps_write!(load_store::reg_Y, am::opc_step_illegal),
    /* 0x8D - abs | STA oper */
    am::absolute::opcode_steps_write!(load_store::reg_A, am::opc_step_illegal),
    /* 0x8E - abs | STX oper */
    am::absolute::opcode_steps_write!(load_store::reg_X, am::opc_step_illegal),
    /* 0x8F - abs | SAX oper */
    am::absolute::opcode_steps_write!(load_store::A_AND_X, am::opc_step_illegal),
    /* 0x90 - rel | BCC oper */
    am::relative::opcode_steps!(control_flow_branch::BCC_core, am::opc_step_illegal),
    /* 0x91 - (ind),Y | STA (oper),Y */
    am::post_indexed_indirect::opcode_steps_write!(load_store::reg_A, load_store::reg_Y, am::opc_step_illegal),
    /* 0x92 */ am::stub_opcode_steps!(),
    /* 0x93 */ am::stub_opcode_steps!(),
    /* 0x94 - zpg,X | STY oper,X */
    am::indexed_zero_page::opcode_steps_write!(load_store::reg_Y, load_store::reg_X, am::opc_step_illegal),
    /* 0x95 - zpg,X | STA oper,X */
    am::indexed_zero_page::opcode_steps_write!(load_store::reg_A, load_store::reg_X, am::opc_step_illegal),
    /* 0x96 - zpg,Y | STX oper,Y */
    am::indexed_zero_page::opcode_steps_write!(load_store::reg_X, load_store::reg_Y, am::opc_step_illegal),
    /* 0x97 - zpg,Y | SAX oper,Y */
    am::indexed_zero_page::opcode_steps_write!(load_store::A_AND_X, load_store::reg_Y, am::opc_step_illegal),
    /* 0x98 - impl | TYA */
    am::implied::opcode_steps!(transfer::TYA_core, am::opc_step_illegal),
    /* 0x99 - abs,Y | STA oper,Y */
    am::indexed_absolute::opcode_steps_write!(load_store::reg_A, load_store::reg_Y, am::opc_step_illegal),
    /* 0x9A - impl | TXS */
    am::implied::opcode_steps!(transfer::TXS_core, am::opc_step_illegal),
    /* 0x9B */ am::stub_opcode_steps!(),
    /* 0x9C */ am::stub_opcode_steps!(),
    /* 0x9D - abs,X | STA oper,X */
    am::indexed_absolute::opcode_steps_write!(load_store::reg_A, load_store::reg_X, am::opc_step_illegal),
    /* 0x9E */ am::stub_opcode_steps!(),
    /* 0x9F */ am::stub_opcode_steps!(),
    /* 0xA0 - # | LDY #oper */
    am::immediate::opcode_steps!(load_store::set_reg_Y, am::opc_step_illegal),
    /* 0xA1 - (ind,X) | LDA (oper,X) */
    am::pre_indexed_indirect::opcode_steps_read!(load_store::set_reg_A, load_store::reg_X, am::opc_step_illegal),
    /* 0xA2 - # | LDX #oper */
    am::immediate::opcode_steps!(load_store::set_reg_X, am::opc_step_illegal),
    /* 0xA3 - (ind,X) | LAX (oper,X) */
    am::pre_indexed_indirect::opcode_steps_read!(load_store::set_regs_AX, load_store::reg_X, am::opc_step_illegal),
    /* 0xA4 - zpg | LDY oper */
    am::zero_page::opcode_steps_read!(load_store::set_reg_Y, am::opc_step_illegal),
    /* 0xA5 - zpg | LDA oper */
    am::zero_page::opcode_steps_read!(load_store::set_reg_A, am::opc_step_illegal),
    /* 0xA6 - zpg | LDX oper */
    am::zero_page::opcode_steps_read!(load_store::set_reg_X, am::opc_step_illegal),
    /* 0xA7 - zpg | LAX oper */
    am::zero_page::opcode_steps_read!(load_store::set_regs_AX, am::opc_step_illegal),
    /* 0xA8 - impl | TAY */
    am::implied::opcode_steps!(transfer::TAY_core, am::opc_step_illegal),
    /* 0xA9 - # | LDA #oper */
    am::immediate::opcode_steps!(load_store::set_reg_A, am::opc_step_illegal),
    /* 0xAA - impl | TAX */
    am::implied::opcode_steps!(transfer::TAX_core, am::opc_step_illegal),
    /* 0xAB - # | LAX #oper */
    am::immediate::opcode_steps!(load_store::set_regs_AX, am::opc_step_illegal),
    /* 0xAC - abs | LDY oper */
    am::absolute::opcode_steps_read!(load_store::set_reg_Y, am::opc_step_illegal),
    /* 0xAD - abs | LDA oper */
    am::absolute::opcode_steps_read!(load_store::set_reg_A, am::opc_step_illegal),
    /* 0xAE - abs | LDX oper */
    am::absolute::opcode_steps_read!(load_store::set_reg_X, am::opc_step_illegal),
    /* 0xAF - abs | LAX oper */
    am::absolute::opcode_steps_read!(load_store::set_regs_AX, am::opc_step_illegal),
    /* 0xB0 - rel | BCS oper */
    am::relative::opcode_steps!(control_flow_branch::BCS_core, am::opc_step_illegal),
    /* 0xB1 - (ind),Y | LDA (oper),Y */
    am::post_indexed_indirect::opcode_steps_read!(load_store::set_reg_A, load_store::reg_Y, am::opc_step_illegal),
    /* 0xB2 */ am::stub_opcode_steps!(),
    /* 0xB3 - (ind),Y | LAX (oper),Y */
    am::post_indexed_indirect::opcode_steps_read!(load_store::set_regs_AX, load_store::reg_Y, am::opc_step_illegal),
    /* 0xB4 - zpg,X | LDY oper,X */
    am::indexed_zero_page::opcode_steps_read!(load_store::set_reg_Y, load_store::reg_X, am::opc_step_illegal),
    /* 0xB5 - zpg,X | LDA oper,X */
    am::indexed_zero_page::opcode_steps_read!(load_store::set_reg_A, load_store::reg_X, am::opc_step_illegal),
    /* 0xB6 - zpg,Y | LDX oper,Y */
    am::indexed_zero_page::opcode_steps_read!(load_store::set_reg_X, load_store::reg_Y, am::opc_step_illegal),
    /* 0xB7 - zpg,Y | LAX oper,Y */
    am::indexed_zero_page::opcode_steps_read!(load_store::set_regs_AX, load_store::reg_Y, am::opc_step_illegal),
    /* 0xB8 - impl | CLV */
    am::implied::opcode_steps!(flags::CLV_core, am::opc_step_illegal),
    /* 0xB9 - abs,Y | LDA oper,Y */
    am::indexed_absolute::opcode_steps_read!(load_store::set_reg_A, load_store::reg_Y, am::opc_step_illegal),
    /* 0xBA - impl | TSX */
    am::implied::opcode_steps!(transfer::TSX_core, am::opc_step_illegal),
    /* 0xBB */ am::stub_opcode_steps!(),
    /* 0xBC - abs,X | LDY oper,X */
    am::indexed_absolute::opcode_steps_read!(load_store::set_reg_Y, load_store::reg_X, am::opc_step_illegal),
    /* 0xBD - abs,X | LDA oper,X */
    am::indexed_absolute::opcode_steps_read!(load_store::set_reg_A, load_store::reg_X, am::opc_step_illegal),
    /* 0xBE - abs,Y | LDX oper,Y */
    am::indexed_absolute::opcode_steps_read!(load_store::set_reg_X, load_store::reg_Y, am::opc_step_illegal),
    /* 0xBF - abs,Y | LAX oper,Y */
    am::indexed_absolute::opcode_steps_read!(load_store::set_regs_AX, load_store::reg_Y, am::opc_step_illegal),
    /* 0xC0 - # | CPY #oper */
    am::immediate::opcode_steps!(arithmetic::CPY_core, am::opc_step_illegal),
    /* 0xC1 - (ind,X) | CMP (oper,X) */
    am::pre_indexed_indirect::opcode_steps_read!(arithmetic::CMP_A_core, load_store::reg_X, am::opc_step_illegal),
    /* 0xC2 - # | NOP #oper */
    am::immediate::opcode_steps!(nop::NOP_2, am::opc_step_illegal),
    /* 0xC3 - (ind,X) | DCP (oper,X) */
    am::pre_indexed_indirect::opcode_steps_read_modify_write!(arithmetic::DCP_core, load_store::reg_X, am::opc_step_illegal),
    /* 0xC4 - zpg | CPY oper */
    am::zero_page::opcode_steps_read!(arithmetic::CMP_Y_core, am::opc_step_illegal),
    /* 0xC5 - zpg | CMP oper */
    am::zero_page::opcode_steps_read!(arithmetic::CMP_A_core, am::opc_step_illegal),
    /* 0xC6 - zpg | DEC oper */
    am::zero_page::opcode_steps_read_modify_write!(arithmetic_inc_dec::DEC_core, am::opc_step_illegal),
    /* 0xC7 - zpg | DCP oper */
    am::zero_page::opcode_steps_read_modify_write!(arithmetic::DCP_core, am::opc_step_illegal),
    /* 0xC8 - impl | INY */
    am::implied::opcode_steps!(arithmetic_inc_dec::INY_core, am::opc_step_illegal),
    /* 0xC9 - # | CMP #oper */
    am::immediate::opcode_steps!(arithmetic::CMP_A_core, am::opc_step_illegal),
    /* 0xCA - impl | DEX */
    am::implied::opcode_steps!(arithmetic_inc_dec::DEX_core, am::opc_step_illegal),
    /* 0xCB */ am::stub_opcode_steps!(),
    /* 0xCC - abs | CPY oper */
    am::absolute::opcode_steps_read!(arithmetic::CMP_Y_core, am::opc_step_illegal),
    /* 0xCD - abs | CMP oper */
    am::absolute::opcode_steps_read!(arithmetic::CMP_A_core, am::opc_step_illegal),
    /* 0xCE - abs | DEC oper */
    am::absolute::opcode_steps_read_modify_write!(arithmetic_inc_dec::DEC_core, am::opc_step_illegal),
    /* 0xCF - abs | DCP oper */
    am::absolute::opcode_steps_read_modify_write!(arithmetic::DCP_core, am::opc_step_illegal),
    /* 0xD0 - rel | BNE oper */
    am::relative::opcode_steps!(control_flow_branch::BNE_core, am::opc_step_illegal),
    /* 0xD1 - (ind),Y | CMP (oper),Y */
    am::post_indexed_indirect::opcode_steps_read!(arithmetic::CMP_A_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0xD2 */ am::stub_opcode_steps!(),
    /* 0xD3 - (ind),Y | DCP (oper),Y */
    am::post_indexed_indirect::opcode_steps_read_modify_write!(arithmetic::DCP_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0xD4 - zpg,X | NOP oper,X */
    am::indexed_zero_page::opcode_steps_read!(nop::NOP_2, load_store::reg_X, am::opc_step_illegal),
    /* 0xD5 - zpg,X | CMP oper,X */
    am::indexed_zero_page::opcode_steps_read!(arithmetic::CMP_A_core, load_store::reg_X, am::opc_step_illegal),
    /* 0xD6 - zpg,X | DEC oper,X */
    am::indexed_zero_page::opcode_steps_read_modify_write!(arithmetic_inc_dec::DEC_core, load_store::reg_X, am::opc_step_illegal),
    /* 0xD7 - zpg,X | DCP oper,X */
    am::indexed_zero_page::opcode_steps_read_modify_write!(arithmetic::DCP_core, load_store::reg_X, am::opc_step_illegal),
    /* 0xD8 - impl | CLD */
    am::implied::opcode_steps!(flags::CLD_core, am::opc_step_illegal),
    /* 0xD9 - abs,Y | CMP oper,Y */
    am::indexed_absolute::opcode_steps_read!(arithmetic::CMP_A_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0xDA - impl | NOP */
    am::implied::opcode_steps!(nop::NOP_1, am::opc_step_illegal),
    /* 0xDB - abs,Y | DCP oper,Y */
    am::indexed_absolute::opcode_steps_read_modify_write!(arithmetic::DCP_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0xDC - abs,X | NOP oper,X */
    am::indexed_absolute::opcode_steps_read!(nop::NOP_2, load_store::reg_X, am::opc_step_illegal),
    /* 0xDD - abs,X | CMP oper,X */
    am::indexed_absolute::opcode_steps_read!(arithmetic::CMP_A_core, load_store::reg_X, am::opc_step_illegal),
    /* 0xDE - abs,X | DEC oper,X */
    am::indexed_absolute::opcode_steps_read_modify_write!(arithmetic_inc_dec::DEC_core, load_store::reg_X, am::opc_step_illegal),
    /* 0xDF - abs,X | DCP oper,X */
    am::indexed_absolute::opcode_steps_read_modify_write!(arithmetic::DCP_core, load_store::reg_X, am::opc_step_illegal),
    /* 0xE0 - # | CPX #oper */
    am::immediate::opcode_steps!(arithmetic::CPX_core, am::opc_step_illegal),
    /* 0xE1 - (ind,X) | SBC (oper,X) */
    am::pre_indexed_indirect::opcode_steps_read!(arithmetic::SBC_core, load_store::reg_X, am::opc_step_illegal),
    /* 0xE2 - # | NOP #oper */
    am::immediate::opcode_steps!(nop::NOP_2, am::opc_step_illegal),
    /* 0xE3 - (ind,X) | ISC (oper,X) */
    am::pre_indexed_indirect::opcode_steps_read_modify_write!(arithmetic::ISC_core, load_store::reg_X, am::opc_step_illegal),
    /* 0xE4 - zpg | CPX oper */
    am::zero_page::opcode_steps_read!(arithmetic::CMP_X_core, am::opc_step_illegal),
    /* 0xE5 - zpg | SBC oper */
    am::zero_page::opcode_steps_read!(arithmetic::SBC_core, am::opc_step_illegal),
    /* 0xE6 - zpg | INC oper */
    am::zero_page::opcode_steps_read_modify_write!(arithmetic_inc_dec::INC_core, am::opc_step_illegal),
    /* 0xE7 - zpg | ISC oper */
    am::zero_page::opcode_steps_read_modify_write!(arithmetic::ISC_core, am::opc_step_illegal),
    /* 0xE8 - impl | INX */
    am::implied::opcode_steps!(arithmetic_inc_dec::INX_core, am::opc_step_illegal),
    /* 0xE9 - # | SBC #oper */
    am::immediate::opcode_steps!(arithmetic::SBC_core, am::opc_step_illegal),
    /* 0xEA - impl | NOP */
    am::implied::opcode_steps!(nop::NOP_1, am::opc_step_illegal),
    /* 0xEB */ am::stub_opcode_steps!(),
    /* 0xEC - abs | CPX oper */
    am::absolute::opcode_steps_read!(arithmetic::CMP_X_core, am::opc_step_illegal),
    /* 0xED - abs | SBC oper */
    am::absolute::opcode_steps_read!(arithmetic::SBC_core, am::opc_step_illegal),
    /* 0xEE - abs | INC oper */
    am::absolute::opcode_steps_read_modify_write!(arithmetic_inc_dec::INC_core, am::opc_step_illegal),
    /* 0xEF - abs | ISC oper */
    am::absolute::opcode_steps_read_modify_write!(arithmetic::ISC_core, am::opc_step_illegal),
    /* 0xF0 - rel | BEQ oper */
    am::relative::opcode_steps!(control_flow_branch::BEQ_core, am::opc_step_illegal),
    /* 0xF1 - (ind),Y | SBC (oper),Y */
    am::post_indexed_indirect::opcode_steps_read!(arithmetic::SBC_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0xF2 */ am::stub_opcode_steps!(),
    /* 0xF3 - (ind),Y | ISC (oper),Y */
    am::post_indexed_indirect::opcode_steps_read_modify_write!(arithmetic::ISC_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0xF4 - zpg,X | NOP oper,X */
    am::indexed_zero_page::opcode_steps_read!(nop::NOP_2, load_store::reg_X, am::opc_step_illegal),
    /* 0xF5 - zpg,X | SBC oper,X */
    am::indexed_zero_page::opcode_steps_read!(arithmetic::SBC_core, load_store::reg_X, am::opc_step_illegal),
    /* 0xF6 - zpg,X | INC oper,X */
    am::indexed_zero_page::opcode_steps_read_modify_write!(arithmetic_inc_dec::INC_core, load_store::reg_X, am::opc_step_illegal),
    /* 0xF7 - zpg,X | ISC oper,X */
    am::indexed_zero_page::opcode_steps_read_modify_write!(arithmetic::ISC_core, load_store::reg_X, am::opc_step_illegal),
    /* 0xF8 - impl | SED */
    am::implied::opcode_steps!(flags::SED_core, am::opc_step_illegal),
    /* 0xF9 - abs,Y | SBC oper,Y */
    am::indexed_absolute::opcode_steps_read!(arithmetic::SBC_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0xFA - impl | NOP */
    am::implied::opcode_steps!(nop::NOP_1, am::opc_step_illegal),
    /* 0xFB - abs,Y | ISC oper,Y */
    am::indexed_absolute::opcode_steps_read_modify_write!(arithmetic::ISC_core, load_store::reg_Y, am::opc_step_illegal),
    /* 0xFC - abs,X | NOP oper,X */
    am::indexed_absolute::opcode_steps_read!(nop::NOP_2, load_store::reg_X, am::opc_step_illegal),
    /* 0xFD - abs,X | SBC oper,X */
    am::indexed_absolute::opcode_steps_read!(arithmetic::SBC_core, load_store::reg_X, am::opc_step_illegal),
    /* 0xFE - abs,X | INC oper,X */
    am::indexed_absolute::opcode_steps_read_modify_write!(arithmetic_inc_dec::INC_core, load_store::reg_X, am::opc_step_illegal),
    /* 0xFF - abs,X | ISC oper,X */
    am::indexed_absolute::opcode_steps_read_modify_write!(arithmetic::ISC_core, load_store::reg_X, am::opc_step_illegal),
];
