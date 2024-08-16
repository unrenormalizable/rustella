use super::{cpu::*, mem::Memory};

// TODO: Safe + and - ops.

/// References (use multiple to cross check implementation):
/// - https://www.masswerk.at/6502/6502_instruction_set.html
/// - https://www.pagetable.com/c64ref/6502/
type OpCode = dyn Fn(u8, u8, u8, &mut MCS6502, &mut Memory);

fn todo(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    todo!("TBD: pcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn illegal(opc: u8, pc_lo: u8, pc_hi: u8, _: &mut MCS6502, _: &mut Memory) {
    panic!("Illegal opcode {} @ {}{}", opc, pc_hi, pc_lo)
}

fn nop(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.pc_incr(1)
}

fn sei(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.set_psr_bit(PSR::I);
    cpu.pc_incr(1)
}

fn cld(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.clr_psr_bit(PSR::D);
    cpu.pc_incr(1)
}

fn ldx_imme(_: u8, pc_lo: u8, pc_hi: u8, cpu: &mut MCS6502, mem: &mut Memory) {
    let val = mem.get(pc_lo + 1, pc_hi);
    cpu.set_x(val);

    _sync_pcr_n(cpu, val);
    _sync_pcr_z(cpu, val);

    cpu.pc_incr(2)
}

fn txs(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let x = cpu.x();
    cpu.set_s(x);
    cpu.pc_incr(1)
}

fn sec(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.set_psr_bit(PSR::C);
    cpu.pc_incr(1)
}

fn sed(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.set_psr_bit(PSR::D);
    cpu.pc_incr(1)
}

fn clc(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.clr_psr_bit(PSR::C);
    cpu.pc_incr(1)
}

fn cli(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.clr_psr_bit(PSR::I);
    cpu.pc_incr(1)
}

fn clv(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.clr_psr_bit(PSR::V);
    cpu.pc_incr(1)
}

fn txa(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let x = cpu.x();
    cpu.set_a(x);

    _sync_pcr_n(cpu, x);
    _sync_pcr_z(cpu, x);

    cpu.pc_incr(1);
}

fn tsx(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let s = cpu.s();
    cpu.set_x(s);

    _sync_pcr_n(cpu, s);
    _sync_pcr_z(cpu, s);

    cpu.pc_incr(1);
}

fn tya(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let y = cpu.y();
    cpu.set_a(y);

    _sync_pcr_n(cpu, y);
    _sync_pcr_z(cpu, y);

    cpu.pc_incr(1);
}

fn tax(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let a = cpu.a();
    cpu.set_x(a);

    _sync_pcr_n(cpu, a);
    _sync_pcr_z(cpu, a);

    cpu.pc_incr(1);
}

fn tay(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let a = cpu.a();
    cpu.set_y(a);

    _sync_pcr_n(cpu, a);
    _sync_pcr_z(cpu, a);

    cpu.pc_incr(1);
}

fn pha(_: u8, _: u8, _: u8, cpu: &mut MCS6502, mem: &mut Memory) {
    let a = cpu.a();
    _push(cpu, mem, a);

    cpu.pc_incr(1);
}

fn php(_: u8, _: u8, _: u8, cpu: &mut MCS6502, mem: &mut Memory) {
    let p = cpu.p();
    _push(cpu, mem, p);

    cpu.pc_incr(1);
}

fn pla(_: u8, _: u8, _: u8, cpu: &mut MCS6502, mem: &mut Memory) {
    let val = _pop(cpu, mem);
    cpu.set_a(val);

    _sync_pcr_n(cpu, val);
    _sync_pcr_z(cpu, val);

    cpu.pc_incr(1);
}

fn plp(_: u8, _: u8, _: u8, cpu: &mut MCS6502, mem: &mut Memory) {
    let val = _pop(cpu, mem);
    cpu.set_p(val);

    cpu.pc_incr(1);
}

fn asl_a(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let old_a = cpu.a();
    let new_a = old_a << 1;

    _sync_pcr_n(cpu, new_a);
    _sync_pcr_z(cpu, new_a);
    _sync_pcr_c_msb(cpu, old_a);

    cpu.set_a(new_a);

    cpu.pc_incr(1);
}

fn lsr_a(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let old_a = cpu.a();
    let new_a = old_a >> 1;

    cpu.clr_psr_bit(PSR::N);
    _sync_pcr_z(cpu, new_a);
    _sync_pcr_c_lsb(cpu, old_a);

    cpu.set_a(new_a);

    cpu.pc_incr(1);
}

fn rol(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.pc_incr(1);
    todo!();
}

fn ror(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.pc_incr(1);
    todo!();
}

fn rti(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.pc_incr(1);
    todo!();
}

fn rts(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    cpu.pc_incr(1);
    todo!();
}

fn dey(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let val = _safe_sub(cpu.y(), 1).0;
    cpu.set_y(val);

    _sync_pcr_n(cpu, val);
    _sync_pcr_z(cpu, val);

    cpu.pc_incr(1);
}

fn iny(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let val = _safe_add(cpu.y(), 1).0;
    cpu.set_y(val);

    _sync_pcr_n(cpu, val);
    _sync_pcr_z(cpu, val);

    cpu.pc_incr(1);
}

fn dex(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let val = _safe_sub(cpu.x(), 1).0;
    cpu.set_x(val);

    _sync_pcr_n(cpu, val);
    _sync_pcr_z(cpu, val);

    cpu.pc_incr(1);
}

fn inx(_: u8, _: u8, _: u8, cpu: &mut MCS6502, _: &mut Memory) {
    let val = _safe_add(cpu.x(), 1).0;
    cpu.set_x(val);

    _sync_pcr_n(cpu, val);
    _sync_pcr_z(cpu, val);

    cpu.pc_incr(1);
}

fn _push(cpu: &mut MCS6502, mem: &mut Memory, val: u8) {
    mem.set(cpu.s(), 0x00, val);

    let s = _safe_sub(cpu.s(), 1).0;
    cpu.set_s(s);
}

fn _pop(cpu: &mut MCS6502, mem: &mut Memory) -> u8 {
    let s = _safe_add(cpu.s(), 1).0;
    cpu.set_s(s);

    mem.get(s, 0x00)
}

fn _safe_add(val1: u8, val2: u8) -> (u8, bool) {
    let res = val1 as u16 + val2 as u16;

    let v = res & 0b1_0000_0000 != 0;

    (res as u8, v)
}

fn _safe_sub(val1: u8, val2: u8) -> (u8, bool) {
    let res = val1 as i16 - val2 as i16;

    let v = res & 0b1_0000_0000 != 0;

    (res as u8, v)
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

/// NOTE: See opc.json
#[rustfmt::skip]
pub const ALL_OPCODE_ROUTINES: &[&OpCode; 0x1_00] = &[
    /* 0x00 */ &todo,
    /* 0x01 */ &todo,
    /* 0x02 */ &illegal,
    /* 0x03 */ &illegal,
    /* 0x04 */ &illegal,
    /* 0x05 */ &todo,
    /* 0x06 */ &todo,
    /* 0x07 */ &illegal,
    /* 0x08 */ &php,
    /* 0x09 */ &todo,
    /* 0x0A */ &asl_a,
    /* 0x0B */ &illegal,
    /* 0x0C */ &illegal,
    /* 0x0D */ &todo,
    /* 0x0E */ &todo,
    /* 0x0F */ &illegal,
    /* 0x10 */ &todo,
    /* 0x11 */ &todo,
    /* 0x12 */ &illegal,
    /* 0x13 */ &illegal,
    /* 0x14 */ &illegal,
    /* 0x15 */ &todo,
    /* 0x16 */ &todo,
    /* 0x17 */ &illegal,
    /* 0x18 */ &clc,
    /* 0x19 */ &todo,
    /* 0x1A */ &illegal,
    /* 0x1B */ &illegal,
    /* 0x1C */ &illegal,
    /* 0x1D */ &todo,
    /* 0x1E */ &todo,
    /* 0x1F */ &illegal,
    /* 0x20 */ &todo,
    /* 0x21 */ &todo,
    /* 0x22 */ &illegal,
    /* 0x23 */ &illegal,
    /* 0x24 */ &todo,
    /* 0x25 */ &todo,
    /* 0x26 */ &todo,
    /* 0x27 */ &illegal,
    /* 0x28 */ &plp,
    /* 0x29 */ &todo,
    /* 0x2A */ &rol,
    /* 0x2B */ &illegal,
    /* 0x2C */ &todo,
    /* 0x2D */ &todo,
    /* 0x2E */ &todo,
    /* 0x2F */ &illegal,
    /* 0x30 */ &todo,
    /* 0x31 */ &todo,
    /* 0x32 */ &illegal,
    /* 0x33 */ &illegal,
    /* 0x34 */ &illegal,
    /* 0x35 */ &todo,
    /* 0x36 */ &todo,
    /* 0x37 */ &illegal,
    /* 0x38 */ &sec,
    /* 0x39 */ &todo,
    /* 0x3A */ &illegal,
    /* 0x3B */ &illegal,
    /* 0x3C */ &illegal,
    /* 0x3D */ &todo,
    /* 0x3E */ &todo,
    /* 0x3F */ &illegal,
    /* 0x40 */ &rti,
    /* 0x41 */ &todo,
    /* 0x42 */ &illegal,
    /* 0x43 */ &illegal,
    /* 0x44 */ &illegal,
    /* 0x45 */ &todo,
    /* 0x46 */ &todo,
    /* 0x47 */ &illegal,
    /* 0x48 */ &pha,
    /* 0x49 */ &todo,
    /* 0x4A */ &lsr_a,
    /* 0x4B */ &illegal,
    /* 0x4C */ &todo,
    /* 0x4D */ &todo,
    /* 0x4E */ &todo,
    /* 0x4F */ &illegal,
    /* 0x50 */ &todo,
    /* 0x51 */ &todo,
    /* 0x52 */ &illegal,
    /* 0x53 */ &illegal,
    /* 0x54 */ &illegal,
    /* 0x55 */ &todo,
    /* 0x56 */ &todo,
    /* 0x57 */ &illegal,
    /* 0x58 */ &cli,
    /* 0x59 */ &todo,
    /* 0x5A */ &illegal,
    /* 0x5B */ &illegal,
    /* 0x5C */ &illegal,
    /* 0x5D */ &todo,
    /* 0x5E */ &todo,
    /* 0x5F */ &illegal,
    /* 0x60 */ &rts,
    /* 0x61 */ &todo,
    /* 0x62 */ &illegal,
    /* 0x63 */ &illegal,
    /* 0x64 */ &illegal,
    /* 0x65 */ &todo,
    /* 0x66 */ &todo,
    /* 0x67 */ &illegal,
    /* 0x68 */ &pla,
    /* 0x69 */ &todo,
    /* 0x6A */ &ror,
    /* 0x6B */ &illegal,
    /* 0x6C */ &todo,
    /* 0x6D */ &todo,
    /* 0x6E */ &todo,
    /* 0x6F */ &illegal,
    /* 0x70 */ &todo,
    /* 0x71 */ &todo,
    /* 0x72 */ &illegal,
    /* 0x73 */ &illegal,
    /* 0x74 */ &illegal,
    /* 0x75 */ &todo,
    /* 0x76 */ &todo,
    /* 0x77 */ &illegal,
    /* 0x78 */ &sei,
    /* 0x79 */ &todo,
    /* 0x7A */ &illegal,
    /* 0x7B */ &illegal,
    /* 0x7C */ &illegal,
    /* 0x7D */ &todo,
    /* 0x7E */ &todo,
    /* 0x7F */ &illegal,
    /* 0x80 */ &illegal,
    /* 0x81 */ &todo,
    /* 0x82 */ &illegal,
    /* 0x83 */ &illegal,
    /* 0x84 */ &todo,
    /* 0x85 */ &todo,
    /* 0x86 */ &todo,
    /* 0x87 */ &illegal,
    /* 0x88 */ &dey,
    /* 0x89 */ &illegal,
    /* 0x8A */ &txa,
    /* 0x8B */ &illegal,
    /* 0x8C */ &todo,
    /* 0x8D */ &todo,
    /* 0x8E */ &todo,
    /* 0x8F */ &illegal,
    /* 0x90 */ &todo,
    /* 0x91 */ &todo,
    /* 0x92 */ &illegal,
    /* 0x93 */ &illegal,
    /* 0x94 */ &todo,
    /* 0x95 */ &todo,
    /* 0x96 */ &todo,
    /* 0x97 */ &illegal,
    /* 0x98 */ &tya,
    /* 0x99 */ &todo,
    /* 0x9A */ &txs,
    /* 0x9B */ &illegal,
    /* 0x9C */ &illegal,
    /* 0x9D */ &todo,
    /* 0x9E */ &illegal,
    /* 0x9F */ &illegal,
    /* 0xA0 */ &todo,
    /* 0xA1 */ &todo,
    /* 0xA2 */ &ldx_imme,
    /* 0xA3 */ &illegal,
    /* 0xA4 */ &todo,
    /* 0xA5 */ &todo,
    /* 0xA6 */ &todo,
    /* 0xA7 */ &illegal,
    /* 0xA8 */ &tay,
    /* 0xA9 */ &todo,
    /* 0xAA */ &tax,
    /* 0xAB */ &illegal,
    /* 0xAC */ &todo,
    /* 0xAD */ &todo,
    /* 0xAE */ &todo,
    /* 0xAF */ &illegal,
    /* 0xB0 */ &todo,
    /* 0xB1 */ &todo,
    /* 0xB2 */ &illegal,
    /* 0xB3 */ &illegal,
    /* 0xB4 */ &todo,
    /* 0xB5 */ &todo,
    /* 0xB6 */ &todo,
    /* 0xB7 */ &illegal,
    /* 0xB8 */ &clv,
    /* 0xB9 */ &todo,
    /* 0xBA */ &tsx,
    /* 0xBB */ &illegal,
    /* 0xBC */ &todo,
    /* 0xBD */ &todo,
    /* 0xBE */ &todo,
    /* 0xBF */ &illegal,
    /* 0xC0 */ &todo,
    /* 0xC1 */ &todo,
    /* 0xC2 */ &illegal,
    /* 0xC3 */ &illegal,
    /* 0xC4 */ &todo,
    /* 0xC5 */ &todo,
    /* 0xC6 */ &todo,
    /* 0xC7 */ &illegal,
    /* 0xC8 */ &iny,
    /* 0xC9 */ &todo,
    /* 0xCA */ &dex,
    /* 0xCB */ &illegal,
    /* 0xCC */ &todo,
    /* 0xCD */ &todo,
    /* 0xCE */ &todo,
    /* 0xCF */ &illegal,
    /* 0xD0 */ &todo,
    /* 0xD1 */ &todo,
    /* 0xD2 */ &illegal,
    /* 0xD3 */ &illegal,
    /* 0xD4 */ &illegal,
    /* 0xD5 */ &todo,
    /* 0xD6 */ &todo,
    /* 0xD7 */ &illegal,
    /* 0xD8 */ &cld,
    /* 0xD9 */ &todo,
    /* 0xDA */ &illegal,
    /* 0xDB */ &illegal,
    /* 0xDC */ &illegal,
    /* 0xDD */ &todo,
    /* 0xDE */ &todo,
    /* 0xDF */ &illegal,
    /* 0xE0 */ &todo,
    /* 0xE1 */ &todo,
    /* 0xE2 */ &illegal,
    /* 0xE3 */ &illegal,
    /* 0xE4 */ &todo,
    /* 0xE5 */ &todo,
    /* 0xE6 */ &todo,
    /* 0xE7 */ &illegal,
    /* 0xE8 */ &inx,
    /* 0xE9 */ &todo,
    /* 0xEA */ &nop,
    /* 0xEB */ &illegal,
    /* 0xEC */ &todo,
    /* 0xED */ &todo,
    /* 0xEE */ &todo,
    /* 0xEF */ &illegal,
    /* 0xF0 */ &todo,
    /* 0xF1 */ &todo,
    /* 0xF2 */ &illegal,
    /* 0xF3 */ &illegal,
    /* 0xF4 */ &illegal,
    /* 0xF5 */ &todo,
    /* 0xF6 */ &todo,
    /* 0xF7 */ &illegal,
    /* 0xF8 */ &sed,
    /* 0xF9 */ &todo,
    /* 0xFA */ &illegal,
    /* 0xFB */ &illegal,
    /* 0xFC */ &illegal,
    /* 0xFD */ &todo,
    /* 0xFE */ &todo,
    /* 0xFF */ &illegal,
];

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0x10, 0x50, (0x60, false))]
    #[test_case(0xfe, 0x01, (0xff, false))]
    #[test_case(0xff, 0x01, (0x00, true))]
    #[test_case(0xfe, 0x11, (0x0f, true))]
    fn test_safe_add(v1: u8, v2: u8, exp: (u8, bool)) {
        let obt = _safe_add(v1, v2);
        assert_eq!(exp, obt);
    }

    #[test_case(0x10, 0x10, (0x00, false))]
    #[test_case(0x00, 0x01, (0xFF, true))]
    #[test_case(0x10, 0x20, (0xF0, true))]
    fn test_safe_sub(v1: u8, v2: u8, exp: (u8, bool)) {
        let obt = _safe_sub(v1, v2);
        assert_eq!(exp, obt);
    }

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
