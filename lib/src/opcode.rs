#![allow(non_snake_case)]

use super::{cmn, cpu::*, mem::Memory};

// TODO: Safe + and - ops.

/// References (use multiple to cross check implementation):
/// - https://www.masswerk.at/6502/6502_instruction_set.html
/// - https://www.pagetable.com/c64ref/6502/
type OpCode = dyn Fn(u8, u8, u8, &mut MCS6502, &mut Memory);

fn illegal(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    panic!("Illegal opcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn BRK_impl(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ORA_ind_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ORA_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ASL_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn PHP_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, mem: &mut Memory) {
    let p = cpu.p();
    _push(cpu, mem, p);
}

fn ORA_imme(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ASL_A(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let old_a = cpu.a();
    let new_a = old_a << 1;

    _sync_pcr_n(cpu, new_a);
    _sync_pcr_z(cpu, new_a);
    _sync_pcr_c_msb(cpu, old_a);

    cpu.set_a(new_a);
}

fn ORA_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ASL_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn BPL_rel(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ORA_ind_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ORA_zpg_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ASL_zpg_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn CLC_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.clr_psr_bit(PSR::C);
}

fn ORA_abs_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ORA_abs_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ASL_abs_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn JSR_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn AND_ind_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn BIT_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn AND_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ROL_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn PLP_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, mem: &mut Memory) {
    let val = _pop(cpu, mem);
    cpu.set_p(val);
}

fn AND_imme(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ROL_A(_: u8, _: u8, _: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!();
}

fn BIT_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn AND_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ROL_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn BMI_rel(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn AND_ind_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn AND_zpg_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ROL_zpg_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn SEC_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.set_psr_bit(PSR::C);
}

fn AND_abs_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn AND_abs_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ROL_abs_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn RTI_impl(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn EOR_ind_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn EOR_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LSR_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn PHA_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, mem: &mut Memory) {
    let a = cpu.a();
    _push(cpu, mem, a);
}

fn EOR_imme(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LSR_A(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let old_a = cpu.a();
    let new_a = old_a >> 1;

    cpu.clr_psr_bit(PSR::N);
    _sync_pcr_z(cpu, new_a);
    _sync_pcr_c_lsb(cpu, old_a);

    cpu.set_a(new_a);
}

fn JMP_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn EOR_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LSR_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn BVC_rel(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn EOR_ind_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn EOR_zpg_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LSR_zpg_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn CLI_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.clr_psr_bit(PSR::I);
}

fn EOR_abs_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn EOR_abs_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LSR_abs_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn RTS_impl(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ADC_ind_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ADC_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ROR_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn PLA_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, mem: &mut Memory) {
    let val = _pop(cpu, mem);
    cpu.set_a(val);

    _sync_pcr_n(cpu, val);
    _sync_pcr_z(cpu, val);
}

fn ADC_imme(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ROR_A(_: u8, _: u8, _: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!();
}

fn JMP_ind(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ADC_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ROR_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn BVS_rel(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ADC_ind_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ADC_zpg_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ROR_zpg_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn SEI_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.set_psr_bit(PSR::I);
}

fn ADC_abs_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ADC_abs_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn ROR_abs_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn STA_ind_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn STY_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn STA_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn STX_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn DEY_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let val = cmn::safe_sub(cpu.y(), 1).0;
    cpu.set_y(val);

    _sync_pcr_n(cpu, val);
    _sync_pcr_z(cpu, val);
}

fn TXA_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let x = cpu.x();
    cpu.set_a(x);

    _sync_pcr_n(cpu, x);
    _sync_pcr_z(cpu, x);
}

fn STY_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn STA_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn STX_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn BCC_rel(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn STA_ind_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn STY_zpg_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn STA_zpg_X(_: u8, pc_lo: u8, pc_hi: u8, cpu: &mut MCS6502, mem: &mut Memory) {
    let val = mem.get(pc_lo + 1, pc_hi);
    let lo = cmn::safe_add(val, cpu.x()).0;
    mem.set(lo, 0x00, cpu.a());
}

fn STX_zpg_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn TYA_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let y = cpu.y();
    cpu.set_a(y);

    _sync_pcr_n(cpu, y);
    _sync_pcr_z(cpu, y);
}

fn STA_abs_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn TXS_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let x = cpu.x();
    cpu.set_s(x);
}

fn STA_abs_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LDY_imme(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LDA_ind_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LDX_imme(_: u8, pc_lo: u8, pc_hi: u8, cpu: &mut MCS6502, mem: &mut Memory) {
    // TODO: SAFE ADD here and all other places by using mandatory offset.
    let val = mem.get(pc_lo + 1, pc_hi);
    cpu.set_x(val);

    _sync_pcr_n(cpu, val);
    _sync_pcr_z(cpu, val);
}

fn LDY_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LDA_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LDX_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn TAY_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let a = cpu.a();
    cpu.set_y(a);

    _sync_pcr_n(cpu, a);
    _sync_pcr_z(cpu, a);
}

fn LDA_imme(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn TAX_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let a = cpu.a();
    cpu.set_x(a);

    _sync_pcr_n(cpu, a);
    _sync_pcr_z(cpu, a);
}

fn LDY_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LDA_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LDX_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn BCS_rel(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LDA_ind_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LDY_zpg_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LDA_zpg_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LDX_zpg_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn CLV_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.clr_psr_bit(PSR::V);
}

fn LDA_abs_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn TSX_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let s = cpu.s();
    cpu.set_x(s);

    _sync_pcr_n(cpu, s);
    _sync_pcr_z(cpu, s);
}

fn LDY_abs_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LDA_abs_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn LDX_abs_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn CPY_imme(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn CMP_ind_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn CPY_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn CMP_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn DEC_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn INY_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let val = cmn::safe_add(cpu.y(), 1).0;
    cpu.set_y(val);

    _sync_pcr_n(cpu, val);
    _sync_pcr_z(cpu, val);
}

fn CMP_imme(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn DEX_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let val = cmn::safe_sub(cpu.x(), 1).0;
    cpu.set_x(val);

    _sync_pcr_n(cpu, val);
    _sync_pcr_z(cpu, val);
}

fn CPY_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn CMP_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn DEC_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn BNE_rel(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn CMP_ind_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn CMP_zpg_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn DEC_zpg_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn CLD_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.clr_psr_bit(PSR::D);
}

fn CMP_abs_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn CMP_abs_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn DEC_abs_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn CPX_imme(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn SBC_ind_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn CPX_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn SBC_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn INC_zpg(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn INX_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let val = cmn::safe_add(cpu.x(), 1).0;
    cpu.set_x(val);

    _sync_pcr_n(cpu, val);
    _sync_pcr_z(cpu, val);
}

fn SBC_imme(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn NOP_impl(_: u8, _: u8, _: u8, _: &mut MCS6502, _: &mut Memory) {}

fn CPX_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn SBC_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn INC_abs(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn BEQ_rel(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn SBC_ind_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn SBC_zpg_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn INC_zpg_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn SED_impl(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.set_psr_bit(PSR::D);
}

fn SBC_abs_Y(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn SBC_abs_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn INC_abs_X(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn _push(cpu: &mut MCS6502, mem: &mut Memory, val: u8) {
    mem.set(cpu.s(), 0x00, val);

    let s = cmn::safe_sub(cpu.s(), 1).0;
    cpu.set_s(s);
}

fn _pop(cpu: &mut MCS6502, mem: &mut Memory) -> u8 {
    let s = cmn::safe_add(cpu.s(), 1).0;
    cpu.set_s(s);

    mem.get(s, 0x00)
}

fn __sync_pcr_c(cpu: &mut MCS6502, val: u8, bit_selector: u8) {
    if tst_bit(val, bit_selector) {
        cpu.set_psr_bit(PSR::C)
    } else {
        cpu.clr_psr_bit(PSR::C)
    }
}

fn _sync_pcr_c_lsb(cpu: &mut MCS6502, val: u8) {
    __sync_pcr_c(cpu, val, 0b0000_0001);
}

fn _sync_pcr_c_msb(cpu: &mut MCS6502, val: u8) {
    __sync_pcr_c(cpu, val, 0b1000_0000);
}

fn _sync_pcr_z(cpu: &mut MCS6502, val: u8) {
    if val == 0 {
        cpu.set_psr_bit(PSR::Z)
    } else {
        cpu.clr_psr_bit(PSR::Z)
    }
}

fn _sync_pcr_n(cpu: &mut MCS6502, val: u8) {
    if tst_bit(val, 0b1000_0000) {
        cpu.set_psr_bit(PSR::N)
    } else {
        cpu.clr_psr_bit(PSR::N)
    }
}

fn _rotate_left(val: u8) -> u8 {
    val.rotate_left(1)
}

fn _rotate_right(val: u8) -> u8 {
    val.rotate_right(1)
}

/*
To regenerated this run
$map = @{}; gc -Raw "D:\src\u\a2600\lib\src\opcodes.json" | ConvertFrom-Json | sort -Property opc | % { $map[$_.opc] = '/* 0x{0:x2} */ &{1}_{2},' -f ($_.opc, $_.assembler.split(" ")[0], $_.addressing.replace("(", "").replace(")", "").replace(",", "_").replace("#", "imme")) }
0..0xff | % { $opc = "{0:X2}" -f $_; if ($map.Contains($opc)) { "    {0}" -f $map[$opc] } else { '    /* 0x{0} */ &illegal,' -f $opc } }

To regenerate the function stubs run
$opc_fns = 0..0xff | % { $opc = "{0:X2}" -f $_; if ($map.Contains($opc)) { "    {0}" -f $map[$opc] } else { '    /* 0x{0} */ &illegal,' -f $opc } }
$opc_fns | ? { !$_.EndsWith("&illegal,") } | % { $_.Substring(16).replace(",", "") } | % { "fn CLI_impl(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {`n    todo!(`"TBD: pcode {} @ {}{}`", opc, pc_hi, pc_lo)`n}`n" }
*/
/// NOTE: See opcodes.json
#[rustfmt::skip]
pub const ALL_OPCODE_ROUTINES: &[&OpCode; 0x1_00] = &[
    /* 0x00 */ &BRK_impl,
    /* 0x01 */ &ORA_ind_X,
    /* 0x02 */ &illegal,
    /* 0x03 */ &illegal,
    /* 0x04 */ &illegal,
    /* 0x05 */ &ORA_zpg,
    /* 0x06 */ &ASL_zpg,
    /* 0x07 */ &illegal,
    /* 0x08 */ &PHP_impl,
    /* 0x09 */ &ORA_imme,
    /* 0x0A */ &ASL_A,
    /* 0x0B */ &illegal,
    /* 0x0C */ &illegal,
    /* 0x0D */ &ORA_abs,
    /* 0x0E */ &ASL_abs,
    /* 0x0F */ &illegal,
    /* 0x10 */ &BPL_rel,
    /* 0x11 */ &ORA_ind_Y,
    /* 0x12 */ &illegal,
    /* 0x13 */ &illegal,
    /* 0x14 */ &illegal,
    /* 0x15 */ &ORA_zpg_X,
    /* 0x16 */ &ASL_zpg_X,
    /* 0x17 */ &illegal,
    /* 0x18 */ &CLC_impl,
    /* 0x19 */ &ORA_abs_Y,
    /* 0x1A */ &illegal,
    /* 0x1B */ &illegal,
    /* 0x1C */ &illegal,
    /* 0x1D */ &ORA_abs_X,
    /* 0x1E */ &ASL_abs_X,
    /* 0x1F */ &illegal,
    /* 0x20 */ &JSR_abs,
    /* 0x21 */ &AND_ind_X,
    /* 0x22 */ &illegal,
    /* 0x23 */ &illegal,
    /* 0x24 */ &BIT_zpg,
    /* 0x25 */ &AND_zpg,
    /* 0x26 */ &ROL_zpg,
    /* 0x27 */ &illegal,
    /* 0x28 */ &PLP_impl,
    /* 0x29 */ &AND_imme,
    /* 0x2A */ &ROL_A,
    /* 0x2B */ &illegal,
    /* 0x2C */ &BIT_abs,
    /* 0x2D */ &AND_abs,
    /* 0x2E */ &ROL_abs,
    /* 0x2F */ &illegal,
    /* 0x30 */ &BMI_rel,
    /* 0x31 */ &AND_ind_Y,
    /* 0x32 */ &illegal,
    /* 0x33 */ &illegal,
    /* 0x34 */ &illegal,
    /* 0x35 */ &AND_zpg_X,
    /* 0x36 */ &ROL_zpg_X,
    /* 0x37 */ &illegal,
    /* 0x38 */ &SEC_impl,
    /* 0x39 */ &AND_abs_Y,
    /* 0x3A */ &illegal,
    /* 0x3B */ &illegal,
    /* 0x3C */ &illegal,
    /* 0x3D */ &AND_abs_X,
    /* 0x3E */ &ROL_abs_X,
    /* 0x3F */ &illegal,
    /* 0x40 */ &RTI_impl,
    /* 0x41 */ &EOR_ind_X,
    /* 0x42 */ &illegal,
    /* 0x43 */ &illegal,
    /* 0x44 */ &illegal,
    /* 0x45 */ &EOR_zpg,
    /* 0x46 */ &LSR_zpg,
    /* 0x47 */ &illegal,
    /* 0x48 */ &PHA_impl,
    /* 0x49 */ &EOR_imme,
    /* 0x4A */ &LSR_A,
    /* 0x4B */ &illegal,
    /* 0x4C */ &JMP_abs,
    /* 0x4D */ &EOR_abs,
    /* 0x4E */ &LSR_abs,
    /* 0x4F */ &illegal,
    /* 0x50 */ &BVC_rel,
    /* 0x51 */ &EOR_ind_Y,
    /* 0x52 */ &illegal,
    /* 0x53 */ &illegal,
    /* 0x54 */ &illegal,
    /* 0x55 */ &EOR_zpg_X,
    /* 0x56 */ &LSR_zpg_X,
    /* 0x57 */ &illegal,
    /* 0x58 */ &CLI_impl,
    /* 0x59 */ &EOR_abs_Y,
    /* 0x5A */ &illegal,
    /* 0x5B */ &illegal,
    /* 0x5C */ &illegal,
    /* 0x5D */ &EOR_abs_X,
    /* 0x5E */ &LSR_abs_X,
    /* 0x5F */ &illegal,
    /* 0x60 */ &RTS_impl,
    /* 0x61 */ &ADC_ind_X,
    /* 0x62 */ &illegal,
    /* 0x63 */ &illegal,
    /* 0x64 */ &illegal,
    /* 0x65 */ &ADC_zpg,
    /* 0x66 */ &ROR_zpg,
    /* 0x67 */ &illegal,
    /* 0x68 */ &PLA_impl,
    /* 0x69 */ &ADC_imme,
    /* 0x6A */ &ROR_A,
    /* 0x6B */ &illegal,
    /* 0x6C */ &JMP_ind,
    /* 0x6D */ &ADC_abs,
    /* 0x6E */ &ROR_abs,
    /* 0x6F */ &illegal,
    /* 0x70 */ &BVS_rel,
    /* 0x71 */ &ADC_ind_Y,
    /* 0x72 */ &illegal,
    /* 0x73 */ &illegal,
    /* 0x74 */ &illegal,
    /* 0x75 */ &ADC_zpg_X,
    /* 0x76 */ &ROR_zpg_X,
    /* 0x77 */ &illegal,
    /* 0x78 */ &SEI_impl,
    /* 0x79 */ &ADC_abs_Y,
    /* 0x7A */ &illegal,
    /* 0x7B */ &illegal,
    /* 0x7C */ &illegal,
    /* 0x7D */ &ADC_abs_X,
    /* 0x7E */ &ROR_abs_X,
    /* 0x7F */ &illegal,
    /* 0x80 */ &illegal,
    /* 0x81 */ &STA_ind_X,
    /* 0x82 */ &illegal,
    /* 0x83 */ &illegal,
    /* 0x84 */ &STY_zpg,
    /* 0x85 */ &STA_zpg,
    /* 0x86 */ &STX_zpg,
    /* 0x87 */ &illegal,
    /* 0x88 */ &DEY_impl,
    /* 0x89 */ &illegal,
    /* 0x8A */ &TXA_impl,
    /* 0x8B */ &illegal,
    /* 0x8C */ &STY_abs,
    /* 0x8D */ &STA_abs,
    /* 0x8E */ &STX_abs,
    /* 0x8F */ &illegal,
    /* 0x90 */ &BCC_rel,
    /* 0x91 */ &STA_ind_Y,
    /* 0x92 */ &illegal,
    /* 0x93 */ &illegal,
    /* 0x94 */ &STY_zpg_X,
    /* 0x95 */ &STA_zpg_X,
    /* 0x96 */ &STX_zpg_Y,
    /* 0x97 */ &illegal,
    /* 0x98 */ &TYA_impl,
    /* 0x99 */ &STA_abs_Y,
    /* 0x9A */ &TXS_impl,
    /* 0x9B */ &illegal,
    /* 0x9C */ &illegal,
    /* 0x9D */ &STA_abs_X,
    /* 0x9E */ &illegal,
    /* 0x9F */ &illegal,
    /* 0xA0 */ &LDY_imme,
    /* 0xA1 */ &LDA_ind_X,
    /* 0xA2 */ &LDX_imme,
    /* 0xA3 */ &illegal,
    /* 0xA4 */ &LDY_zpg,
    /* 0xA5 */ &LDA_zpg,
    /* 0xA6 */ &LDX_zpg,
    /* 0xA7 */ &illegal,
    /* 0xA8 */ &TAY_impl,
    /* 0xA9 */ &LDA_imme,
    /* 0xAA */ &TAX_impl,
    /* 0xAB */ &illegal,
    /* 0xAC */ &LDY_abs,
    /* 0xAD */ &LDA_abs,
    /* 0xAE */ &LDX_abs,
    /* 0xAF */ &illegal,
    /* 0xB0 */ &BCS_rel,
    /* 0xB1 */ &LDA_ind_Y,
    /* 0xB2 */ &illegal,
    /* 0xB3 */ &illegal,
    /* 0xB4 */ &LDY_zpg_X,
    /* 0xB5 */ &LDA_zpg_X,
    /* 0xB6 */ &LDX_zpg_Y,
    /* 0xB7 */ &illegal,
    /* 0xB8 */ &CLV_impl,
    /* 0xB9 */ &LDA_abs_Y,
    /* 0xBA */ &TSX_impl,
    /* 0xBB */ &illegal,
    /* 0xBC */ &LDY_abs_X,
    /* 0xBD */ &LDA_abs_X,
    /* 0xBE */ &LDX_abs_Y,
    /* 0xBF */ &illegal,
    /* 0xC0 */ &CPY_imme,
    /* 0xC1 */ &CMP_ind_X,
    /* 0xC2 */ &illegal,
    /* 0xC3 */ &illegal,
    /* 0xC4 */ &CPY_zpg,
    /* 0xC5 */ &CMP_zpg,
    /* 0xC6 */ &DEC_zpg,
    /* 0xC7 */ &illegal,
    /* 0xC8 */ &INY_impl,
    /* 0xC9 */ &CMP_imme,
    /* 0xCA */ &DEX_impl,
    /* 0xCB */ &illegal,
    /* 0xCC */ &CPY_abs,
    /* 0xCD */ &CMP_abs,
    /* 0xCE */ &DEC_abs,
    /* 0xCF */ &illegal,
    /* 0xD0 */ &BNE_rel,
    /* 0xD1 */ &CMP_ind_Y,
    /* 0xD2 */ &illegal,
    /* 0xD3 */ &illegal,
    /* 0xD4 */ &illegal,
    /* 0xD5 */ &CMP_zpg_X,
    /* 0xD6 */ &DEC_zpg_X,
    /* 0xD7 */ &illegal,
    /* 0xD8 */ &CLD_impl,
    /* 0xD9 */ &CMP_abs_Y,
    /* 0xDA */ &illegal,
    /* 0xDB */ &illegal,
    /* 0xDC */ &illegal,
    /* 0xDD */ &CMP_abs_X,
    /* 0xDE */ &DEC_abs_X,
    /* 0xDF */ &illegal,
    /* 0xE0 */ &CPX_imme,
    /* 0xE1 */ &SBC_ind_X,
    /* 0xE2 */ &illegal,
    /* 0xE3 */ &illegal,
    /* 0xE4 */ &CPX_zpg,
    /* 0xE5 */ &SBC_zpg,
    /* 0xE6 */ &INC_zpg,
    /* 0xE7 */ &illegal,
    /* 0xE8 */ &INX_impl,
    /* 0xE9 */ &SBC_imme,
    /* 0xEA */ &NOP_impl,
    /* 0xEB */ &illegal,
    /* 0xEC */ &CPX_abs,
    /* 0xED */ &SBC_abs,
    /* 0xEE */ &INC_abs,
    /* 0xEF */ &illegal,
    /* 0xF0 */ &BEQ_rel,
    /* 0xF1 */ &SBC_ind_Y,
    /* 0xF2 */ &illegal,
    /* 0xF3 */ &illegal,
    /* 0xF4 */ &illegal,
    /* 0xF5 */ &SBC_zpg_X,
    /* 0xF6 */ &INC_zpg_X,
    /* 0xF7 */ &illegal,
    /* 0xF8 */ &SED_impl,
    /* 0xF9 */ &SBC_abs_Y,
    /* 0xFA */ &illegal,
    /* 0xFB */ &illegal,
    /* 0xFC */ &illegal,
    /* 0xFD */ &SBC_abs_X,
    /* 0xFE */ &INC_abs_X,
    /* 0xFF */ &illegal,
];

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0b0000_0000, 0b0000_0000)]
    #[test_case(0b0100_0000, 0b1000_0000)]
    #[test_case(0b1000_0000, 0b0000_0001)]
    fn test_rotate_left(v: u8, exp: u8) {
        let obt = _rotate_left(v);
        assert_eq!(exp, obt);
    }

    #[test_case(0b0000_0000, 0b0000_0000)]
    #[test_case(0b0000_0010, 0b0000_0001)]
    #[test_case(0b0000_0001, 0b1000_0000)]
    fn test_rotate_right(v: u8, exp: u8) {
        let obt = _rotate_right(v);
        assert_eq!(exp, obt);
    }

    #[test]
    fn test_push_pop() {
        let mut cpu = MCS6502::new(0x00, 0x00);
        let mut mem = Memory::new(&[0b01010101; 0x1000], true);

        const SP: u8 = 0xff;
        cpu.set_s(SP);
        let val = mem.get(cpu.s(), 0);
        assert_eq!(val, 0x0d);

        _push(&mut cpu, &mut mem, 0x55);
        assert_eq!(cpu.s(), SP - 1);
        assert_eq!(mem.get(SP, 0), 0x55);
        let val = _pop(&mut cpu, &mut mem);
        assert_eq!(val, 0x55);
        assert_eq!(cpu.s(), SP);
    }
}
