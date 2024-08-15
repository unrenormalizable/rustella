use super::{cpu::*, mem::Memory};

/// References (use multiple to cross check implementation):
/// - https://www.masswerk.at/6502/6502_instruction_set.html
/// - https://www.pagetable.com/c64ref/6502/
type OpCode = dyn Fn(&mut MCS6502, &mut Memory) -> (u8, u8);

fn todo(_: &mut MCS6502, _: &mut Memory) -> (u8, u8) {
    todo!()
}

fn illegal(_: &mut MCS6502, _: &mut Memory) -> (u8, u8) {
    panic!("Illegal opcode")
}

fn sei(cpu: &mut MCS6502, _: &mut Memory) -> (u8, u8) {
    cpu.set_psr_bit(PSR::I);
    cpu.pc(1)
}

fn cld(cpu: &mut MCS6502, _: &mut Memory) -> (u8, u8) {
    cpu.clr_psr_bit(PSR::D);
    cpu.pc(1)
}

fn ldx_imme(cpu: &mut MCS6502, mem: &mut Memory) -> (u8, u8) {
    let pc = cpu.pc(0);
    let val = mem.get(pc.0 + 1, pc.1);
    cpu.set_x(val);

    if val == 0 {
        cpu.set_psr_bit(PSR::Z)
    } else {
        cpu.clr_psr_bit(PSR::Z)
    }

    if tst_bit(val, 0b1000_0000) {
        cpu.set_psr_bit(PSR::N)
    } else {
        cpu.clr_psr_bit(PSR::N)
    }

    (pc.0 + 2, pc.1)
}

fn txs(cpu: &mut MCS6502, _: &mut Memory) -> (u8, u8) {
    let x = cpu.x();
    cpu.set_s(x);
    cpu.pc(1)
}

/// NOTE: To generate json data, run the following on https://www.masswerk.at/6502/6502_instruction_set.html
/// JSON.stringify([...document.querySelectorAll("dl[class='opcodes'] [aria-label='details'] tbody")].flatMap(x => [...x.querySelectorAll("tr")].slice(1).map(e => { const fields = [...e.childNodes].map(c => c.innerText); const obj = {}; obj["addressing"] = fields[0].trim(); obj["assembler"] = fields[1].trim(); obj["opc"] = fields[2].trim(); obj["bytes"] = fields[3].trim(); obj["cycles"] = fields[4].trim(); return obj })))
/// gc -Raw D:\src\delme\opc.json | ConvertFrom-Json | sort -Property opc | ConvertTo-Json
pub const ALL_OPCODES: &[&OpCode; 0x1_00] = &[
    /* 0x00 */ &todo, /* 0x01 */ &todo, /* 0x02 */ &illegal,
    /* 0x03 */ &illegal, /* 0x04 */ &illegal, /* 0x05 */ &todo,
    /* 0x06 */ &todo, /* 0x07 */ &illegal, /* 0x08 */ &todo, /* 0x09 */ &todo,
    /* 0x0A */ &todo, /* 0x0B */ &illegal, /* 0x0C */ &illegal,
    /* 0x0D */ &todo, /* 0x0E */ &todo, /* 0x0F */ &illegal, /* 0x10 */ &todo,
    /* 0x11 */ &todo, /* 0x12 */ &illegal, /* 0x13 */ &illegal,
    /* 0x14 */ &illegal, /* 0x15 */ &todo, /* 0x16 */ &todo,
    /* 0x17 */ &illegal, /* 0x18 */ &todo, /* 0x19 */ &todo,
    /* 0x1A */ &illegal, /* 0x1B */ &illegal, /* 0x1C */ &illegal,
    /* 0x1D */ &todo, /* 0x1E */ &todo, /* 0x1F */ &illegal, /* 0x20 */ &todo,
    /* 0x21 */ &todo, /* 0x22 */ &illegal, /* 0x23 */ &illegal,
    /* 0x24 */ &todo, /* 0x25 */ &todo, /* 0x26 */ &todo, /* 0x27 */ &illegal,
    /* 0x28 */ &todo, /* 0x29 */ &todo, /* 0x2A */ &todo, /* 0x2B */ &illegal,
    /* 0x2C */ &todo, /* 0x2D */ &todo, /* 0x2E */ &todo, /* 0x2F */ &illegal,
    /* 0x30 */ &todo, /* 0x31 */ &todo, /* 0x32 */ &illegal,
    /* 0x33 */ &illegal, /* 0x34 */ &illegal, /* 0x35 */ &todo,
    /* 0x36 */ &todo, /* 0x37 */ &illegal, /* 0x38 */ &todo, /* 0x39 */ &todo,
    /* 0x3A */ &illegal, /* 0x3B */ &illegal, /* 0x3C */ &illegal,
    /* 0x3D */ &todo, /* 0x3E */ &todo, /* 0x3F */ &illegal, /* 0x40 */ &todo,
    /* 0x41 */ &todo, /* 0x42 */ &illegal, /* 0x43 */ &illegal,
    /* 0x44 */ &illegal, /* 0x45 */ &todo, /* 0x46 */ &todo,
    /* 0x47 */ &illegal, /* 0x48 */ &todo, /* 0x49 */ &todo, /* 0x4A */ &todo,
    /* 0x4B */ &illegal, /* 0x4C */ &todo, /* 0x4D */ &todo, /* 0x4E */ &todo,
    /* 0x4F */ &illegal, /* 0x50 */ &todo, /* 0x51 */ &todo,
    /* 0x52 */ &illegal, /* 0x53 */ &illegal, /* 0x54 */ &illegal,
    /* 0x55 */ &todo, /* 0x56 */ &todo, /* 0x57 */ &illegal, /* 0x58 */ &todo,
    /* 0x59 */ &todo, /* 0x5A */ &illegal, /* 0x5B */ &illegal,
    /* 0x5C */ &illegal, /* 0x5D */ &todo, /* 0x5E */ &todo,
    /* 0x5F */ &illegal, /* 0x60 */ &todo, /* 0x61 */ &todo,
    /* 0x62 */ &illegal, /* 0x63 */ &illegal, /* 0x64 */ &illegal,
    /* 0x65 */ &todo, /* 0x66 */ &todo, /* 0x67 */ &illegal, /* 0x68 */ &todo,
    /* 0x69 */ &todo, /* 0x6A */ &todo, /* 0x6B */ &illegal, /* 0x6C */ &todo,
    /* 0x6D */ &todo, /* 0x6E */ &todo, /* 0x6F */ &illegal, /* 0x70 */ &todo,
    /* 0x71 */ &todo, /* 0x72 */ &illegal, /* 0x73 */ &illegal,
    /* 0x74 */ &illegal, /* 0x75 */ &todo, /* 0x76 */ &todo,
    /* 0x77 */ &illegal, /* 0x78 */ &sei, /* 0x79 */ &todo,
    /* 0x7A */ &illegal, /* 0x7B */ &illegal, /* 0x7C */ &illegal,
    /* 0x7D */ &todo, /* 0x7E */ &todo, /* 0x7F */ &illegal,
    /* 0x80 */ &illegal, /* 0x81 */ &todo, /* 0x82 */ &illegal,
    /* 0x83 */ &illegal, /* 0x84 */ &todo, /* 0x85 */ &todo, /* 0x86 */ &todo,
    /* 0x87 */ &illegal, /* 0x88 */ &todo, /* 0x89 */ &illegal,
    /* 0x8A */ &todo, /* 0x8B */ &illegal, /* 0x8C */ &todo, /* 0x8D */ &todo,
    /* 0x8E */ &todo, /* 0x8F */ &illegal, /* 0x90 */ &todo, /* 0x91 */ &todo,
    /* 0x92 */ &illegal, /* 0x93 */ &illegal, /* 0x94 */ &todo,
    /* 0x95 */ &todo, /* 0x96 */ &todo, /* 0x97 */ &illegal, /* 0x98 */ &todo,
    /* 0x99 */ &todo, /* 0x9A */ &txs, /* 0x9B */ &illegal,
    /* 0x9C */ &illegal, /* 0x9D */ &todo, /* 0x9E */ &illegal,
    /* 0x9F */ &illegal, /* 0xA0 */ &todo, /* 0xA1 */ &todo,
    /* 0xA2 */ &ldx_imme, /* 0xA3 */ &illegal, /* 0xA4 */ &todo,
    /* 0xA5 */ &todo, /* 0xA6 */ &todo, /* 0xA7 */ &illegal, /* 0xA8 */ &todo,
    /* 0xA9 */ &todo, /* 0xAA */ &todo, /* 0xAB */ &illegal, /* 0xAC */ &todo,
    /* 0xAD */ &todo, /* 0xAE */ &todo, /* 0xAF */ &illegal, /* 0xB0 */ &todo,
    /* 0xB1 */ &todo, /* 0xB2 */ &illegal, /* 0xB3 */ &illegal,
    /* 0xB4 */ &todo, /* 0xB5 */ &todo, /* 0xB6 */ &todo, /* 0xB7 */ &illegal,
    /* 0xB8 */ &todo, /* 0xB9 */ &todo, /* 0xBA */ &todo, /* 0xBB */ &illegal,
    /* 0xBC */ &todo, /* 0xBD */ &todo, /* 0xBE */ &todo, /* 0xBF */ &illegal,
    /* 0xC0 */ &todo, /* 0xC1 */ &todo, /* 0xC2 */ &illegal,
    /* 0xC3 */ &illegal, /* 0xC4 */ &todo, /* 0xC5 */ &todo, /* 0xC6 */ &todo,
    /* 0xC7 */ &illegal, /* 0xC8 */ &todo, /* 0xC9 */ &todo, /* 0xCA */ &todo,
    /* 0xCB */ &illegal, /* 0xCC */ &todo, /* 0xCD */ &todo, /* 0xCE */ &todo,
    /* 0xCF */ &illegal, /* 0xD0 */ &todo, /* 0xD1 */ &todo,
    /* 0xD2 */ &illegal, /* 0xD3 */ &illegal, /* 0xD4 */ &illegal,
    /* 0xD5 */ &todo, /* 0xD6 */ &todo, /* 0xD7 */ &illegal, /* 0xD8 */ &cld,
    /* 0xD9 */ &todo, /* 0xDA */ &illegal, /* 0xDB */ &illegal,
    /* 0xDC */ &illegal, /* 0xDD */ &todo, /* 0xDE */ &todo,
    /* 0xDF */ &illegal, /* 0xE0 */ &todo, /* 0xE1 */ &todo,
    /* 0xE2 */ &illegal, /* 0xE3 */ &illegal, /* 0xE4 */ &todo,
    /* 0xE5 */ &todo, /* 0xE6 */ &todo, /* 0xE7 */ &illegal, /* 0xE8 */ &todo,
    /* 0xE9 */ &todo, /* 0xEA */ &todo, /* 0xEB */ &illegal, /* 0xEC */ &todo,
    /* 0xED */ &todo, /* 0xEE */ &todo, /* 0xEF */ &illegal, /* 0xF0 */ &todo,
    /* 0xF1 */ &todo, /* 0xF2 */ &illegal, /* 0xF3 */ &illegal,
    /* 0xF4 */ &illegal, /* 0xF5 */ &todo, /* 0xF6 */ &todo,
    /* 0xF7 */ &illegal, /* 0xF8 */ &todo, /* 0xF9 */ &todo,
    /* 0xFA */ &illegal, /* 0xFB */ &illegal, /* 0xFC */ &illegal,
    /* 0xFD */ &todo, /* 0xFE */ &todo, /* 0xFF */ &illegal,
];
