use super::{cmn::LoHi, mem::Memory};

// NOTE: Naming conventions from "6502 Address Modes in Detail" in https://www.masswerk.at/6502/6502_instruction_set.html#modes

// Example: LDA #$07 - load the literal hexidecimal value "$7" into the accumulator
pub fn load_immediate(mem: &Memory, pc: LoHi) -> u8 {
    mem.get(pc, 1)
}

pub fn load_immediate_2(mem: &Memory, pc: LoHi) -> LoHi {
    (mem.get(pc, 1), mem.get(pc, 2)).into()
}

// Example: LDA $3010 - load the contents of address "$3010" into the accumulato
pub fn load_absolute(mem: &Memory, pc: LoHi) -> u8 {
    load_absolute_indexed(mem, pc, 0)
}

pub fn store_absolute(mem: &mut Memory, pc: LoHi, val: u8) {
    store_absolute_indexed(mem, pc, 0, val)
}

// Example: LDA $80 - load the contents of address "$0080" into the accumulator
pub fn load_zero_page(mem: &Memory, pc: LoHi) -> u8 {
    load_zero_page_indexed(mem, pc, 0)
}

pub fn store_zero_page(mem: &mut Memory, pc: LoHi, val: u8) {
    store_zero_page_indexed(mem, pc, 0, val)
}

// Example: LDA $3120,X - load the contents of address "$3120 + X" into A
pub fn load_absolute_indexed(mem: &Memory, pc: LoHi, index: u8) -> u8 {
    let abs_args = load_immediate_2(mem, pc);
    let addr = abs_args + index;
    mem.get(addr, 0)
}

pub fn store_absolute_indexed(mem: &mut Memory, pc: LoHi, index: u8, val: u8) {
    let abs_args = load_immediate_2(mem, pc);
    let addr = abs_args + index;
    mem.set(addr, 0, val);
}

// Example: LDA $80,X - load the contents of address "$0080 + X" into A
pub fn load_zero_page_indexed(mem: &Memory, pc: LoHi, index: u8) -> u8 {
    let abs_args = load_immediate(mem, pc);
    let addr = LoHi(abs_args, 0) + index;
    mem.get(addr, 0)
}

pub fn store_zero_page_indexed(mem: &mut Memory, pc: LoHi, index: u8, val: u8) {
    let abs_args = load_immediate(mem, pc);
    let addr = LoHi(abs_args, 0) + index;
    mem.set(addr, 0, val)
}

// Example: JMP ($FF82) - jump to address given in addresses "$FF82" and "$FF83"
pub fn load_indirect(mem: &Memory, pc: LoHi) -> LoHi {
    let op_args = load_immediate_2(mem, pc);
    let lo = mem.get(op_args, 0);
    let hi = mem.get(op_args, 1);

    LoHi(lo, hi)
}

/// Example: LDA ($70,X): load the contents of the location given in addresses "$0070+X" and "$0070+1+X" into A
pub fn load_pre_indexed_indirect(mem: &Memory, pc: LoHi, index: u8) -> u8 {
    let lo = load_zero_page_indexed(mem, pc, index);
    let hi = load_zero_page_indexed(mem, pc, index.wrapping_add(1));

    mem.get(LoHi(lo, hi), 0)
}

/// Example: STA ($A2,X): store the contents of A in the location given in addresses "$00A2+X" and "$00A3+X"
pub fn store_pre_indexed_indirect(mem: &mut Memory, pc: LoHi, index: u8, val: u8) {
    let lo = load_zero_page_indexed(mem, pc, index);
    let hi = load_zero_page_indexed(mem, pc, index.wrapping_add(1));

    mem.set(LoHi(lo, hi), 0, val)
}

/// Example: LDA ($70),Y: add the contents of the Y-register to the pointer provided in "$0070" and "$0071" and load the contents of this address into A
pub fn load_post_indexed_indirect(mem: &Memory, pc: LoHi, index: u8) -> u8 {
    let lo = load_zero_page_indexed(mem, pc, 0);
    let hi = load_zero_page_indexed(mem, pc, 1);

    let addr = LoHi(lo, hi) + index;

    mem.get(addr, 0)
}

/// Example: STA ($A2),Y: store the contents of A in the location given by the pointer in "$00A2" and "$00A3" plus the contents of the Y-register
pub fn store_post_indexed_indirect(mem: &mut Memory, pc: LoHi, index: u8, val: u8) {
    let lo = load_zero_page_indexed(mem, pc, 0);
    let hi = load_zero_page_indexed(mem, pc, 1);

    let addr = LoHi(lo, hi) + index;

    mem.set(addr, 0, val)
}

/// Refer: https://www.pagetable.com/c64ref/6502/?tab=3#r8
/// TODO: Basic +/- working but edge cases (-128 to +127) not tested.
pub fn relative(mem: &Memory, pc: LoHi) -> LoHi {
    let off = mem.get(pc, 1);
    let instr_len = 0x02u8; // NOTE: relative is only used in branch opcs, all have length 2.

    (pc + instr_len).wrapping_add_lo(off)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mmaps;
    use test_case::test_case;

    #[test_case(LoHi(0x10, 0xf0))] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(LoHi(0xff, 0xfe))]
    #[test_case(LoHi(0xff, 0xff))]
    #[test_case(LoHi(0x00, 0xff))]
    fn test_load_immediate(pc: LoHi) {
        let mut mem = Memory::new_with_rom(&[], Default::default(), mmaps::mm_6507, true);
        mem.set(pc, 1, 0x07);
        mem.set(pc, 2, 0xf7);

        let obt = load_immediate(&mem, pc);
        assert_eq!(obt, 0x07);

        let obt = load_immediate_2(&mem, pc);
        assert_eq!(obt, (0x07, 0xf7).into());
    }

    #[test_case(LoHi(0x10, 0xf0), 0x34)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(LoHi(0xff, 0xff), 0xff)]
    #[test_case(LoHi(0xff, 0xfe), 0x66)]
    #[test_case(LoHi(0x00, 0xff), 0x96)]
    fn test_load_absolute(pc: LoHi, exp: u8) {
        let mut mem = Memory::new_with_rom(&[], Default::default(), mmaps::mm_6507, true);
        mem.set(pc, 1, 0x10);
        mem.set(pc, 2, 0x30);
        mem.set(LoHi(0x10, 0x30), 0, exp);

        let obt = load_absolute(&mem, pc);

        assert_eq!(obt, exp);
    }

    #[test_case(LoHi(0x10, 0xf0), 0x34)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(LoHi(0xff, 0xff), 0xff)]
    #[test_case(LoHi(0xff, 0xfe), 0x66)]
    #[test_case(LoHi(0x00, 0xff), 0x96)]
    fn test_store_absolute(pc: LoHi, exp: u8) {
        let mut mem = Memory::new_with_rom(&[], Default::default(), mmaps::mm_6507, true);
        mem.set(pc, 1, 0x10);
        mem.set(pc, 2, 0x30);

        store_absolute(&mut mem, pc, exp);
        let obt = mem.get(LoHi(0x10, 0x30), 0);

        assert_eq!(obt, exp);
    }

    #[test_case(LoHi(0x10, 0xf0), 0x34)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(LoHi(0xff, 0xfe), 0xfe)]
    #[test_case(LoHi(0xff, 0xff), 0x66)]
    #[test_case(LoHi(0x00, 0xff), 0x98)]
    fn test_load_zero_page(pc: LoHi, exp: u8) {
        let mut mem = Memory::new_with_rom(&[], Default::default(), mmaps::mm_6507, true);
        mem.set(pc, 1, 0x80);
        mem.set(LoHi(0x80, 0x00), 0, exp);

        let obt = load_zero_page(&mem, pc);

        assert_eq!(obt, exp);
    }

    #[test_case(LoHi(0x10, 0xf0), 0x34)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(LoHi(0xff, 0xfe), 0xfe)]
    #[test_case(LoHi(0xff, 0xff), 0x66)]
    #[test_case(LoHi(0x00, 0xff), 0x98)]
    fn test_store_zero_page(pc: LoHi, exp: u8) {
        let mut mem = Memory::new_with_rom(&[], Default::default(), mmaps::mm_6507, true);
        mem.set(pc, 1, 0x80);

        store_zero_page(&mut mem, pc, exp);
        let obt = mem.get(LoHi(0x80, 0x00), 0);

        assert_eq!(obt, exp);
    }

    #[test_case(LoHi(0x10, 0xf0), 0x78)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(LoHi(0xff, 0xff), 0xff)]
    #[test_case(LoHi(0xff, 0xfe), 0x66)]
    #[test_case(LoHi(0x00, 0xff), 0x96)]
    fn test_load_absolute_indexed(pc: LoHi, exp: u8) {
        let mut mem = Memory::new_with_rom(&[], Default::default(), mmaps::mm_6507, true);
        mem.set(pc, 1, 0x20);
        mem.set(pc, 2, 0x31);
        let index = 0x12;
        mem.set(LoHi(0x32, 0x31), 0, exp);

        let obt = load_absolute_indexed(&mem, pc, index);

        assert_eq!(obt, exp);
    }

    #[test_case(LoHi(0x10, 0xf0), 0x78)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(LoHi(0xff, 0xff), 0xff)]
    #[test_case(LoHi(0xff, 0xfe), 0x66)]
    #[test_case(LoHi(0x00, 0xff), 0x96)]
    fn test_store_absolute_indexed(pc: LoHi, exp: u8) {
        let mut mem = Memory::new_with_rom(&[], Default::default(), mmaps::mm_6507, true);
        mem.set(pc, 1, 0x20);
        mem.set(pc, 2, 0x31);
        let index = 0x12;

        store_absolute_indexed(&mut mem, pc, index, exp);
        let obt = mem.get(LoHi(0x32, 0x31), 0);

        assert_eq!(obt, exp);
    }

    #[test_case(LoHi(0x10, 0xf0), 0x64)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(LoHi(0xff, 0xfe), 0xfe)]
    #[test_case(LoHi(0xff, 0xff), 0x66)]
    #[test_case(LoHi(0x00, 0xff), 0x98)]
    fn test_load_zero_page_indexed(pc: LoHi, exp: u8) {
        let mut mem = Memory::new_with_rom(&[], Default::default(), mmaps::mm_6507, true);
        mem.set(pc, 1, 0x80);
        let index = 0x02;
        mem.set(LoHi(0x82, 0x00), 0, exp);

        let obt = load_zero_page_indexed(&mem, pc, index);

        assert_eq!(obt, exp);
    }

    #[test_case(LoHi(0x10, 0xf0), 0x64)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(LoHi(0xff, 0xfe), 0xfe)]
    #[test_case(LoHi(0xff, 0xff), 0x66)]
    #[test_case(LoHi(0x00, 0xff), 0x98)]
    fn test_store_zero_page_indexed(pc: LoHi, exp: u8) {
        let mut mem = Memory::new_with_rom(&[], Default::default(), mmaps::mm_6507, true);
        mem.set(pc, 1, 0x80);
        let index = 0x02;

        store_zero_page_indexed(&mut mem, pc, index, exp);
        let obt = mem.get(LoHi(0x82, 0x00), 0);

        assert_eq!(obt, exp);
    }

    #[test_case(LoHi(0x10, 0xf0), LoHi(0xc4, 0x80))] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(LoHi(0xff, 0xfe), LoHi(0xff, 0xff))]
    #[test_case(LoHi(0xff, 0xff), LoHi(0xde, 0xad))]
    #[test_case(LoHi(0x00, 0xff), LoHi(0xbe, 0xef))]
    fn test_load_indirect(pc: LoHi, exp: LoHi) {
        let mut mem = Memory::new_with_rom(&[], Default::default(), mmaps::mm_6507, true);
        mem.set(pc, 1, 0x82);
        mem.set(pc, 2, 0xff);
        mem.set(LoHi(0x82, 0xff), 0, exp.0);
        mem.set(LoHi(0x82, 0xff), 1, exp.1);

        let obt = load_indirect(&mem, pc);

        assert_eq!(obt, exp);
    }

    #[test_case(LoHi(0x10, 0xf0), 0x05, 0xA5)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(LoHi(0xff, 0xff), 0x05, 0xA5)]
    #[test_case(LoHi(0x00, 0xff), 0x05, 0xA5)]
    fn test_load_pre_indexed_indirect(pc: LoHi, index: u8, exp: u8) {
        let mut mem = Memory::new_with_rom(&[], Default::default(), mmaps::mm_6507, true);
        mem.set(pc, 1, 0x70);
        mem.set(LoHi(0x75, 0x00), 0, 0x23);
        mem.set(LoHi(0x75, 0x00), 1, 0x30);
        mem.set(LoHi(0x23, 0x30), 0, 0xA5);

        let obt = load_pre_indexed_indirect(&mem, pc, index);

        assert_eq!(obt, exp);
    }

    #[test_case(LoHi(0x10, 0xf0), 0x05, 0xA5)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(LoHi(0xff, 0xff), 0x05, 0xA5)]
    #[test_case(LoHi(0x00, 0xff), 0x05, 0xA5)]
    fn test_store_pre_indexed_indirect(pc: LoHi, index: u8, exp: u8) {
        let mut mem = Memory::new_with_rom(&[], Default::default(), mmaps::mm_6507, true);
        mem.set(pc, 1, 0x70);
        mem.set(LoHi(0x75, 0x00), 0, 0x23);
        mem.set(LoHi(0x75, 0x00), 1, 0x30);

        store_pre_indexed_indirect(&mut mem, pc, index, 0xA5);

        let obt = mem.get(LoHi(0x23, 0x30), 0);
        assert_eq!(obt, exp);
    }

    #[test_case(LoHi(0x10, 0xf0), 0x10, 0x23)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(LoHi(0xff, 0xff), 0x10, 0x23)]
    #[test_case(LoHi(0x00, 0xff), 0x10, 0x23)]
    fn test_load_post_indexed_indirect(pc: LoHi, index: u8, exp: u8) {
        let mut mem = Memory::new_with_rom(&[], Default::default(), mmaps::mm_6507, true);
        mem.set(pc, 1, 0x70);
        mem.set(LoHi(0x70, 0x00), 0, 0x43);
        mem.set(LoHi(0x70, 0x00), 1, 0x35);
        mem.set(LoHi(0x53, 0x35), 0, 0x23);

        let obt = load_post_indexed_indirect(&mem, pc, index);

        assert_eq!(obt, exp);
    }

    #[test_case(LoHi(0x10, 0xf0), 0x10, 0x23)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(LoHi(0xff, 0xff), 0x10, 0x23)]
    #[test_case(LoHi(0x00, 0xff), 0x10, 0x23)]
    fn test_store_post_indexed_indirect(pc: LoHi, index: u8, exp: u8) {
        let mut mem = Memory::new_with_rom(&[], Default::default(), mmaps::mm_6507, true);
        mem.set(pc, 1, 0x70);
        mem.set(LoHi(0x70, 0x00), 0, 0x43);
        mem.set(LoHi(0x70, 0x00), 1, 0x35);

        store_post_indexed_indirect(&mut mem, pc, index, 0x23);

        let obt = mem.get(LoHi(0x53, 0x35), 0);
        assert_eq!(obt, exp);
    }

    #[test_case(LoHi(0x0a, 0xf0), 0xfb, LoHi(0x07, 0xf0); "Jump by -3+len bytes")]
    #[test_case(LoHi(0x00, 0x00), 0x50, LoHi(0x52, 0x00); "Jump by 50+len bytes")]
    #[test_case(LoHi(0xFF, 0x44), 0x02, LoHi(0x03, 0x45); "Example from AllSuiteA.bin 0x44FF")]
    #[test_case(LoHi(0x00, 0x10), 0x03, LoHi(0x05, 0x10); "Example 1 from masswerk 6502_instruction_set")]
    #[test_case(LoHi(0xD4, 0x08), 0xEE, LoHi(0xC4, 0x08); "Example 2 from masswerk 6502_instruction_set")]
    fn test_relative_addr(pc: LoHi, op_arg: u8, exp: LoHi) {
        let mut mem = Memory::new_with_rom(&[], Default::default(), mmaps::mm_6507, true);
        mem.set(pc, 1, op_arg);

        let obt = relative(&mem, pc);

        assert_eq!(exp, obt);
    }

    #[test]
    #[ignore]
    fn expensive_test_3() {
        // code that takes an hour to run
    }

    #[test]
    #[ignore]
    fn failed_test_3() {
        assert_eq!(1, 2)
    }
}
