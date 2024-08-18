use super::{cmn, mem::Memory};

// NOTE: Naming conventions from "6502 Address Modes in Detail" in https://www.masswerk.at/6502/6502_instruction_set.html#modes

// Example: LDA #$07 - load the literal hexidecimal value "$7" into the accumulator
pub fn load_immediate(mem: &Memory, pc_lo: u8, pc_hi: u8) -> u8 {
    mem.get(pc_lo, pc_hi, 1)
}

pub fn load_immediate_2(mem: &Memory, pc_lo: u8, pc_hi: u8) -> (u8, u8) {
    (mem.get(pc_lo, pc_hi, 1), mem.get(pc_lo, pc_hi, 2))
}

// Example: LDA $3010 - load the contents of address "$3010" into the accumulato
pub fn load_absolute(mem: &Memory, pc_lo: u8, pc_hi: u8) -> u8 {
    load_absolute_indexed(mem, pc_lo, pc_hi, 0)
}

pub fn store_absolute(mem: &mut Memory, pc_lo: u8, pc_hi: u8, val: u8) {
    store_absolute_indexed(mem, pc_lo, pc_hi, 0, val)
}

// Example: LDA $80 - load the contents of address "$0080" into the accumulator
pub fn load_zero_page(mem: &Memory, pc_lo: u8, pc_hi: u8) -> u8 {
    load_zero_page_indexed(mem, pc_lo, pc_hi, 0)
}

pub fn store_zero_page(mem: &mut Memory, pc_lo: u8, pc_hi: u8, val: u8) {
    store_zero_page_indexed(mem, pc_lo, pc_hi, 0, val)
}

// Example: LDA $3120,X - load the contents of address "$3120 + X" into A
pub fn load_absolute_indexed(mem: &Memory, pc_lo: u8, pc_hi: u8, index: u8) -> u8 {
    let abs_args = load_immediate_2(mem, pc_lo, pc_hi);
    let addr = cmn::indexed(abs_args.0, abs_args.1, index);
    mem.get(addr.0, addr.1, 0)
}

pub fn store_absolute_indexed(mem: &mut Memory, pc_lo: u8, pc_hi: u8, index: u8, val: u8) {
    let abs_args = load_immediate_2(mem, pc_lo, pc_hi);
    let addr = cmn::indexed(abs_args.0, abs_args.1, index);
    mem.set(addr.0, addr.1, 0, val);
}

// Example: LDA $80,X - load the contents of address "$0080 + X" into A
pub fn load_zero_page_indexed(mem: &Memory, pc_lo: u8, pc_hi: u8, index: u8) -> u8 {
    let abs_args = load_immediate(mem, pc_lo, pc_hi);
    let addr = cmn::indexed(abs_args, 0, index);
    mem.get(addr.0, 0, 0)
}

pub fn store_zero_page_indexed(mem: &mut Memory, pc_lo: u8, pc_hi: u8, index: u8, val: u8) {
    let abs_args = load_immediate(mem, pc_lo, pc_hi);
    let addr = cmn::indexed(abs_args, 0, index);
    mem.set(addr.0, 0, 0, val)
}

// Example: JMP ($FF82) - jump to address given in addresses "$FF82" and "$FF83"
pub fn load_indirect(mem: &Memory, pc_lo: u8, pc_hi: u8) -> (u8, u8) {
    let op_args = load_immediate_2(mem, pc_lo, pc_hi);
    let lo = mem.get(op_args.0, op_args.1, 0);
    let hi = mem.get(op_args.0, op_args.1, 1);

    (lo, hi)
}

/// Example: LDA ($70,X): load the contents of the location given in addresses "$0070+X" and "$0070+1+X" into A
pub fn load_pre_indexed_indirect(mem: &Memory, pc_lo: u8, pc_hi: u8, index: u8) -> u8 {
    let lo = load_zero_page_indexed(mem, pc_lo, pc_hi, index);
    let hi = load_zero_page_indexed(mem, pc_lo, pc_hi, cmn::safe_add(index, 1).0);

    mem.get(lo, hi, 0)
}

/// Example: STA ($A2,X): store the contents of A in the location given in addresses "$00A2+X" and "$00A3+X"
pub fn store_pre_indexed_indirect(mem: &mut Memory, pc_lo: u8, pc_hi: u8, index: u8, val: u8) {
    let lo = load_zero_page_indexed(mem, pc_lo, pc_hi, index);
    let hi = load_zero_page_indexed(mem, pc_lo, pc_hi, cmn::safe_add(index, 1).0);

    mem.set(lo, hi, 0, val)
}

/// Example: LDA ($70),Y: add the contents of the Y-register to the pointer provided in "$0070" and "$0071" and load the contents of this address into A
pub fn load_post_indexed_indirect(mem: &Memory, pc_lo: u8, pc_hi: u8, index: u8) -> u8 {
    let lo = load_zero_page_indexed(mem, pc_lo, pc_hi, 0);
    let hi = load_zero_page_indexed(mem, pc_lo, pc_hi, 1);

    let addr = cmn::indexed(lo, hi, index);

    mem.get(addr.0, addr.1, 0)
}

/// Example: STA ($A2),Y: store the contents of A in the location given by the pointer in "$00A2" and "$00A3" plus the contents of the Y-register
pub fn store_post_indexed_indirect(mem: &mut Memory, pc_lo: u8, pc_hi: u8, index: u8, val: u8) {
    let lo = load_zero_page_indexed(mem, pc_lo, pc_hi, 0);
    let hi = load_zero_page_indexed(mem, pc_lo, pc_hi, 1);

    let addr = cmn::indexed(lo, hi, index);

    mem.set(addr.0, addr.1, 0, val)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0x10, 0xf0)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(0xff, 0xfe)]
    #[test_case(0xff, 0xff)]
    #[test_case(0x00, 0xff)]
    fn test_load_immediate(pc_lo: u8, pc_hi: u8) {
        let mut mem = Memory::new(&[0b10101010; 0x1000], true);
        mem.set(pc_lo, pc_hi, 1, 0x07);
        mem.set(pc_lo, pc_hi, 2, 0xf7);

        let obt = load_immediate(&mem, pc_lo, pc_hi);
        assert_eq!(obt, 0x07);

        let obt = load_immediate_2(&mem, pc_lo, pc_hi);
        assert_eq!(obt, (0x07, 0xf7));
    }

    #[test_case(0x10, 0xf0, 0x34)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(0xff, 0xff, 0xff)]
    #[test_case(0xff, 0xfe, 0x66)]
    #[test_case(0x00, 0xff, 0x96)]
    fn test_load_absolute(pc_lo: u8, pc_hi: u8, exp: u8) {
        let mut mem = Memory::new(&[0b10101010; 0x1000], true);
        mem.set(pc_lo, pc_hi, 1, 0x10);
        mem.set(pc_lo, pc_hi, 2, 0x30);
        mem.set(0x10, 0x30, 0, exp);

        let obt = load_absolute(&mem, pc_lo, pc_hi);

        assert_eq!(obt, exp);
    }

    #[test_case(0x10, 0xf0, 0x34)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(0xff, 0xff, 0xff)]
    #[test_case(0xff, 0xfe, 0x66)]
    #[test_case(0x00, 0xff, 0x96)]
    fn test_store_absolute(pc_lo: u8, pc_hi: u8, exp: u8) {
        let mut mem = Memory::new(&[0b10101010; 0x1000], true);
        mem.set(pc_lo, pc_hi, 1, 0x10);
        mem.set(pc_lo, pc_hi, 2, 0x30);

        store_absolute(&mut mem, pc_lo, pc_hi, exp);
        let obt = mem.get(0x10, 0x30, 0);

        assert_eq!(obt, exp);
    }

    #[test_case(0x10, 0xf0, 0x34)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(0xff, 0xfe, 0xfe)]
    #[test_case(0xff, 0xff, 0x66)]
    #[test_case(0x00, 0xff, 0x98)]
    fn test_load_zero_page(pc_lo: u8, pc_hi: u8, exp: u8) {
        let mut mem = Memory::new(&[0b10101010; 0x1000], true);
        mem.set(pc_lo, pc_hi, 1, 0x80);
        mem.set(0x80, 0x00, 0, exp);

        let obt = load_zero_page(&mem, pc_lo, pc_hi);

        assert_eq!(obt, exp);
    }

    #[test_case(0x10, 0xf0, 0x34)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(0xff, 0xfe, 0xfe)]
    #[test_case(0xff, 0xff, 0x66)]
    #[test_case(0x00, 0xff, 0x98)]
    fn test_store_zero_page(pc_lo: u8, pc_hi: u8, exp: u8) {
        let mut mem = Memory::new(&[0b10101010; 0x1000], true);
        mem.set(pc_lo, pc_hi, 1, 0x80);

        store_zero_page(&mut mem, pc_lo, pc_hi, exp);
        let obt = mem.get(0x80, 0x00, 0);

        assert_eq!(obt, exp);
    }

    #[test_case(0x10, 0xf0, 0x78)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(0xff, 0xff, 0xff)]
    #[test_case(0xff, 0xfe, 0x66)]
    #[test_case(0x00, 0xff, 0x96)]
    fn test_load_absolute_indexed(pc_lo: u8, pc_hi: u8, exp: u8) {
        let mut mem = Memory::new(&[0b10101010; 0x1000], true);
        mem.set(pc_lo, pc_hi, 1, 0x20);
        mem.set(pc_lo, pc_hi, 2, 0x31);
        let index = 0x12;
        mem.set(0x32, 0x31, 0, exp);

        let obt = load_absolute_indexed(&mem, pc_lo, pc_hi, index);

        assert_eq!(obt, exp);
    }

    #[test_case(0x10, 0xf0, 0x78)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(0xff, 0xff, 0xff)]
    #[test_case(0xff, 0xfe, 0x66)]
    #[test_case(0x00, 0xff, 0x96)]
    fn test_store_absolute_indexed(pc_lo: u8, pc_hi: u8, exp: u8) {
        let mut mem = Memory::new(&[0b10101010; 0x1000], true);
        mem.set(pc_lo, pc_hi, 1, 0x20);
        mem.set(pc_lo, pc_hi, 2, 0x31);
        let index = 0x12;

        store_absolute_indexed(&mut mem, pc_lo, pc_hi, index, exp);
        let obt = mem.get(0x32, 0x31, 0);

        assert_eq!(obt, exp);
    }

    #[test_case(0x10, 0xf0, 0x64)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(0xff, 0xfe, 0xfe)]
    #[test_case(0xff, 0xff, 0x66)]
    #[test_case(0x00, 0xff, 0x98)]
    fn test_load_zero_page_indexed(pc_lo: u8, pc_hi: u8, exp: u8) {
        let mut mem = Memory::new(&[0b10101010; 0x1000], true);
        mem.set(pc_lo, pc_hi, 1, 0x80);
        let index = 0x02;
        mem.set(0x82, 0x00, 0, exp);

        let obt = load_zero_page_indexed(&mem, pc_lo, pc_hi, index);

        assert_eq!(obt, exp);
    }

    #[test_case(0x10, 0xf0, 0x64)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(0xff, 0xfe, 0xfe)]
    #[test_case(0xff, 0xff, 0x66)]
    #[test_case(0x00, 0xff, 0x98)]
    fn test_store_zero_page_indexed(pc_lo: u8, pc_hi: u8, exp: u8) {
        let mut mem = Memory::new(&[0b10101010; 0x1000], true);
        mem.set(pc_lo, pc_hi, 1, 0x80);
        let index = 0x02;

        store_zero_page_indexed(&mut mem, pc_lo, pc_hi, index, exp);
        let obt = mem.get(0x82, 0x00, 0);

        assert_eq!(obt, exp);
    }

    #[test_case(0x10, 0xf0, (0xc4, 0x80))] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(0xff, 0xfe, (0xff, 0xff))]
    #[test_case(0xff, 0xff, (0xde, 0xad))]
    #[test_case(0x00, 0xff, (0xbe, 0xef))]
    fn test_load_indirect(pc_lo: u8, pc_hi: u8, exp: (u8, u8)) {
        let mut mem = Memory::new(&[0b10101010; 0x1000], true);
        mem.set(pc_lo, pc_hi, 1, 0x82);
        mem.set(pc_lo, pc_hi, 2, 0xff);
        mem.set(0x82, 0xff, 0, exp.0);
        mem.set(0x82, 0xff, 1, exp.1);

        let obt = load_indirect(&mem, pc_lo, pc_hi);

        assert_eq!(obt, exp);
    }

    #[test_case(0x10, 0xf0, 0x05, 0xA5)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(0xff, 0xff, 0x05, 0xA5)]
    #[test_case(0x00, 0xff, 0x05, 0xA5)]
    fn test_load_pre_indexed_indirect(pc_lo: u8, pc_hi: u8, index: u8, exp: u8) {
        let mut mem = Memory::new(&[0b10101010; 0x1000], true);
        mem.set(pc_lo, pc_hi, 1, 0x70);
        mem.set(0x75, 0x00, 0, 0x23);
        mem.set(0x75, 0x00, 1, 0x30);
        mem.set(0x23, 0x30, 0, 0xA5);

        let obt = load_pre_indexed_indirect(&mem, pc_lo, pc_hi, index);

        assert_eq!(obt, exp);
    }

    #[test_case(0x10, 0xf0, 0x05, 0xA5)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(0xff, 0xff, 0x05, 0xA5)]
    #[test_case(0x00, 0xff, 0x05, 0xA5)]
    fn test_store_pre_indexed_indirect(pc_lo: u8, pc_hi: u8, index: u8, exp: u8) {
        let mut mem = Memory::new(&[0b10101010; 0x1000], true);
        mem.set(pc_lo, pc_hi, 1, 0x70);
        mem.set(0x75, 0x00, 0, 0x23);
        mem.set(0x75, 0x00, 1, 0x30);

        store_pre_indexed_indirect(&mut mem, pc_lo, pc_hi, index, 0xA5);

        let obt = mem.get(0x23, 0x30, 0);
        assert_eq!(obt, exp);
    }

    #[test_case(0x10, 0xf0, 0x10, 0x23)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(0xff, 0xff, 0x10, 0x23)]
    #[test_case(0x00, 0xff, 0x10, 0x23)]
    fn test_load_post_indexed_indirect(pc_lo: u8, pc_hi: u8, index: u8, exp: u8) {
        let mut mem = Memory::new(&[0b10101010; 0x1000], true);
        mem.set(pc_lo, pc_hi, 1, 0x70);
        mem.set(0x70, 0x00, 0, 0x43);
        mem.set(0x70, 0x00, 1, 0x35);
        mem.set(0x53, 0x35, 0, 0x23);

        let obt = load_post_indexed_indirect(&mem, pc_lo, pc_hi, index);

        assert_eq!(obt, exp);
    }

    #[test_case(0x10, 0xf0, 0x10, 0x23)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
    #[test_case(0xff, 0xff, 0x10, 0x23)]
    #[test_case(0x00, 0xff, 0x10, 0x23)]
    fn test_store_post_indexed_indirect(pc_lo: u8, pc_hi: u8, index: u8, exp: u8) {
        let mut mem = Memory::new(&[0b10101010; 0x1000], true);
        mem.set(pc_lo, pc_hi, 1, 0x70);
        mem.set(0x70, 0x00, 0, 0x43);
        mem.set(0x70, 0x00, 1, 0x35);

        store_post_indexed_indirect(&mut mem, pc_lo, pc_hi, index, 0x23);

        let obt = mem.get(0x53, 0x35, 0);
        assert_eq!(obt, exp);
    }
}
