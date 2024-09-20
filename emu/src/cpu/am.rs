use crate::cmn::LoHi;
use crate::riot::Memory;

/*
    Details: "6502 Address Modes in Detail" in https://www.masswerk.at/6502/6502_instruction_set.html#modes
    - Naming conventions from that document
    - Also refer https://www.pagetable.com/c64ref/6502/?tab=3

    - TODO: Combine load store tests.
*/

/// #2 Immediate Addressing | Immediate
///
/// LDA #$07 - load the literal hexidecimal value "$7" into the accumulator
/// ADC #$A0 - add the literal hexidecimal value "$A0" to the accumulator
/// CPX #$32 - compare the X-register to the literal hexidecimal value "$32"
pub mod immediate {
    use super::*;

    #[inline]
    pub fn load(mem: &Memory, pc: LoHi) -> u8 {
        mem.get(pc, 1)
    }

    #[cfg(test)]
    mod tests {
        use crate::riot;
        use test_case::test_case;

        #[test_case((0x07,); "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        fn test_load(op_arg: (u8,)) {
            let mut mem = riot::Memory::new(true);
            let pc = 0x0000u16.into();

            // Setup Opcode.
            mem.set(pc, 0, 0xA9);
            mem.set(pc, 1, op_arg.0);

            let obt = super::load(&mem, pc);
            assert_eq!(obt, op_arg.0);
        }
    }
}

/// #3 Absolute Addressing | Absolute
///
/// LDA $3010 - load the contents of address "$3010" into the accumulator
/// ROL $08A0 - rotate the contents of address "$08A0" left by one position
/// JMP $4000 - jump to (continue with) location "$4000"
pub mod absolute {
    use super::*;

    #[inline]
    pub fn load(mem: &Memory, pc: LoHi) -> u8 {
        indexed_absolute::load(mem, pc, 0)
    }

    /// Used by JMP & JSR
    #[inline]
    pub fn load_lohi(mem: &Memory, pc: LoHi) -> LoHi {
        LoHi(mem.get(pc, 1), mem.get(pc, 2))
    }

    #[inline]
    pub fn store(mem: &mut Memory, pc: LoHi, val: u8) {
        indexed_absolute::store(mem, pc, 0, val)
    }

    #[cfg(test)]
    mod tests {
        use crate::riot;
        use test_case::test_case;

        #[test_case((0x10, 0x30), 0x34; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        fn test_load(op_args: (u8, u8), exp: u8) {
            let mut mem = riot::Memory::new(true);
            let pc = 0x0000u16.into();

            // Setup OpCode.
            mem.set(pc, 0, 0xAD);
            mem.set(pc, 1, op_args.0);
            mem.set(pc, 2, op_args.1);

            // Setup lookup.
            mem.set(op_args.into(), 0, exp);

            let obt = super::load(&mem, pc);

            assert_eq!(obt, exp);
        }

        #[test_case((0x10, 0x30), 0x34; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        fn test_store(op_args: (u8, u8), exp: u8) {
            let mut mem = riot::Memory::new(true);
            let pc = 0x0000u16.into();

            // Setup OpCode.
            mem.set(pc, 0, 0xAD);
            mem.set(pc, 1, op_args.0);
            mem.set(pc, 2, op_args.1);

            super::store(&mut mem, pc, exp);

            let obt = mem.get(op_args.into(), 0);

            assert_eq!(obt, exp);
        }

        #[test_case((0x10, 0x30); "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        fn test_load_lohi(op_args: (u8, u8)) {
            let mut mem = riot::Memory::new(true);
            let pc = 0x0000u16.into();

            // Setup OpCode.
            mem.set(pc, 0, 0xAD);
            mem.set(pc, 1, op_args.0);
            mem.set(pc, 2, op_args.1);

            let obt = super::load_lohi(&mem, pc);

            assert_eq!(obt, op_args.into());
        }
    }
}

/// #4 Zero-Page Addressing | Zero Page
///
/// LDA $80 - load the contents of address "$0080" into the accumulator
/// BIT $A2 - perform bit-test with the contents of address "$00A2"
/// ASL $9A - arithmetic shift left of the contents of location "$009A"
pub mod zero_page {
    use super::*;

    // Example: LDA $80 - load the contents of address "$0080" into the accumulator
    #[inline]
    pub fn load(mem: &Memory, pc: LoHi) -> u8 {
        indexed_zero_page::load(mem, pc, 0)
    }

    #[inline]
    pub fn store(mem: &mut Memory, pc: LoHi, val: u8) {
        indexed_zero_page::store(mem, pc, 0, val)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::riot;
        use test_case::test_case;

        #[test_case((0x80,), LoHi(0x80, 0x00), 0x34; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        fn test_load(op_args: (u8,), lookup: LoHi, exp: u8) {
            let mut mem = riot::Memory::new(true);
            let pc = 0x0000u16.into();

            // Set up opcode.
            mem.set(pc, 0, 0xA5);
            mem.set(pc, 1, op_args.0);

            // Setup lookup.
            mem.set(lookup, 0, exp);

            let obt = super::load(&mem, pc);

            assert_eq!(obt, exp);
        }

        #[test_case(LoHi(0x10, 0xf0), 0x34)] // Example from https://www.masswerk.at/6502/6502_instruction_set.htm
        #[test_case(LoHi(0xff, 0xfe), 0xfe)]
        #[test_case(LoHi(0xff, 0xff), 0x66)]
        #[test_case(LoHi(0x00, 0xff), 0x98)]
        fn test_store(pc: LoHi, exp: u8) {
            let mut mem = riot::Memory::new(true);
            mem.set(pc, 1, 0x80);

            super::store(&mut mem, pc, exp);
            let obt = mem.get(LoHi(0x80, 0x00), 0);

            assert_eq!(obt, exp);
        }
    }
}

/// #5 Indexed Addressing: Absolute,X and Absolute,Y
///
/// LDA $3120,X - load the contents of address "$3120 + X" into A
/// LDX $8240,Y - load the contents of address "$8240 + Y" into X
/// INC $1400,X - increment the contents of address "$1400 + X"
pub mod indexed_absolute {
    use super::*;

    #[inline]
    fn ea(mem: &Memory, pc: LoHi, index: u8) -> LoHi {
        let abs_args = absolute::load_lohi(mem, pc);

        abs_args + index
    }

    // Example: LDA $3120,X - load the contents of address "$3120 + X" into A
    pub fn load(mem: &Memory, pc: LoHi, index: u8) -> u8 {
        let addr = ea(mem, pc, index);

        mem.get(addr, 0)
    }

    pub fn store(mem: &mut Memory, pc: LoHi, index: u8, val: u8) {
        let addr = ea(mem, pc, index);

        mem.set(addr, 0, val);
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::riot;
        use test_case::test_case;

        #[test_case((0x20, 0x31), 0x12, LoHi(0x32, 0x31), 0x78; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        #[test_case((0xFF, 0x31), 0x01, LoHi(0x00, 0x32), 0x78; "Page wrap around")]
        #[test_case((0xFF, 0xFF), 0x01, LoHi(0x00, 0x00), 0x78; "Address space wrap around")]
        fn test_load(op_args: (u8, u8), index: u8, lookup: LoHi, exp: u8) {
            let mut mem = riot::Memory::new(true);
            let pc = 0x0000u16.into();

            // Set up OpCode.
            mem.set(pc, 0, 0xBD);
            mem.set(pc, 1, op_args.0);
            mem.set(pc, 2, op_args.1);

            // Setup lookup.
            mem.set(lookup, 0, exp);

            let obt = super::load(&mem, pc, index);

            assert_eq!(obt, exp);
        }

        #[test_case((0x20, 0x31), 0x12, LoHi(0x32, 0x31), 0x78; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        #[test_case((0xFF, 0x31), 0x01, LoHi(0x00, 0x32), 0x78; "Page wrap around")]
        #[test_case((0xFF, 0xFF), 0x01, LoHi(0x00, 0x00), 0x78; "Address space wrap around")]
        fn test_store(op_args: (u8, u8), index: u8, lookup: LoHi, exp: u8) {
            let mut mem = riot::Memory::new(true);
            let pc = 0x0000u16.into();

            // Set up OpCode.
            mem.set(pc, 0, 0xBD);
            mem.set(pc, 1, op_args.0);
            mem.set(pc, 2, op_args.1);

            super::store(&mut mem, pc, index, exp);

            let obt = mem.get(lookup, 0);
            assert_eq!(obt, exp);
        }
    }
}

/// #6 Indexed Addressing: Zero-Page,X and Zero-Page,Y
///
/// LDA $80,X - load the contents of address "$0080 + X" into A
/// LSR $82,X - shift the contents of address "$0082 + X" left
/// LDX $60,Y - load the contents of address "$0060 + Y" into X
pub mod indexed_zero_page {
    use super::*;

    #[inline]
    fn ea(mem: &Memory, pc: LoHi, index: u8) -> LoHi {
        let abs_args = immediate::load(mem, pc);
        LoHi(abs_args.wrapping_add(index), 0x00)
    }

    // Example: LDA $80,X - load the contents of address "$0080 + X" into A
    pub fn load(mem: &Memory, pc: LoHi, index: u8) -> u8 {
        let addr = ea(mem, pc, index);

        mem.get(addr, 0x00)
    }

    pub fn store(mem: &mut Memory, pc: LoHi, index: u8, val: u8) {
        let addr = ea(mem, pc, index);

        mem.set(addr, 0x00, val)
    }

    #[cfg(test)]
    mod tests {
        use crate::riot;
        use test_case::test_case;

        #[test_case((0x80,), 0x02, (0x82, 0x00), 0x64; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        #[test_case((0xFF,), 0x01, (0x00, 0x00), 0x64; "Page wrap around")]
        fn test_load(op_args: (u8,), index: u8, lookup: (u8, u8), exp: u8) {
            let mut mem = riot::Memory::new(true);
            let pc = 0x0400u16.into();

            // Setup OpCode.
            mem.set(pc, 0, 0xB6);
            mem.set(pc, 1, op_args.0);

            // Setup lookup.
            mem.set(lookup.into(), 0, exp);

            let obt = super::load(&mem, pc, index);

            assert_eq!(obt, exp);
        }

        #[test_case((0x80,), 0x02, (0x82, 0x00), 0x64; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        #[test_case((0xFF,), 0x01, (0x00, 0x00), 0x64; "Page wrap around")]
        fn test_store(op_args: (u8,), index: u8, lookup: (u8, u8), exp: u8) {
            let mut mem = riot::Memory::new(true);
            let pc = 0x0400u16.into();

            // Setup OpCode.
            mem.set(pc, 0, 0xB6);
            mem.set(pc, 1, op_args.0);

            super::store(&mut mem, pc, index, exp);

            let obt = mem.get(lookup.into(), 0);

            assert_eq!(obt, exp);
        }
    }
}

/// #7 Indirect Addressing | Absolute Indirect
/// On 6502, the indirect jump instruction does not increment the page address when the indirect
///   pointer crosses a page boundary.
///
/// JMP ($xxFF) - jump to address given in addresses $xxFF and $xx00"
pub mod indirect {
    use super::*;

    // Example: JMP ($xxFF) - jump to address given in addresses "$xxFF" and "$xx00"
    pub fn load(mem: &Memory, pc: LoHi) -> LoHi {
        let op_args = absolute::load_lohi(mem, pc);
        let lo = mem.get(LoHi(op_args.0, op_args.1), 0);
        let hi = mem.get(LoHi(op_args.0.wrapping_add(1), op_args.1), 0);

        LoHi(lo, hi)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::riot;
        use test_case::test_case;

        //            Op Args   ->              Lookup             ->     Eff. Addr
        #[test_case((0x82, 0xFF), LoHi(0x82, 0xFF), LoHi(0x83, 0xFF), LoHi(0xC4, 0x80); "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        #[test_case((0xFF, 0xFF), LoHi(0xFF, 0xFF), LoHi(0x00, 0xFF), LoHi(0xC4, 0x80); "No page wrap around for lookup address")]
        fn test_load(op_args: (u8, u8), lookup_lo: LoHi, lookup_hi: LoHi, exp: LoHi) {
            let mut mem = riot::Memory::new(true);
            let pc = 0x0000u16.into();

            // Set up opcode: JMP ($xxFF)
            mem.set(pc, 0, 0x6C);
            mem.set(pc, 1, op_args.0);
            mem.set(pc, 2, op_args.1);

            // Set up ($xxFF) & ($xx00) pointing to effective addr.
            mem.set(lookup_lo, 0, exp.0);
            mem.set(lookup_hi, 0, exp.1);

            let obt = super::load(&mem, pc);

            assert_eq!(obt, exp);
        }
    }
}

/// #8 Pre-Indexed Indirect, "(Zero-Page,X)"
///
/// LDA ($70,X) - load the contents of the location given in addresses "$0070+X" and "$0070+1+X" into A
/// STA ($A2,X) - store the contents of A in the location given in addresses "$00A2+X" and "$00A3+X"
/// EOR ($BA,X) - perform an exlusive OR of the contents of A and the contents of the location given in addresses "$00BA+X" and "$00BB+X"
pub mod pre_indexed_indirect {
    use super::*;

    #[inline]
    fn ea(mem: &Memory, pc: LoHi, index: u8) -> LoHi {
        let lo = indexed_zero_page::load(mem, pc, index);
        let hi = indexed_zero_page::load(mem, pc, index.wrapping_add(1));

        LoHi(lo, hi)
    }

    /// Example: LDA ($70,X): load the contents of the location given in addresses "$0070+X" and "$0070+1+X" into A
    pub fn load(mem: &Memory, pc: LoHi, index: u8) -> u8 {
        let addr = ea(mem, pc, index);

        mem.get(addr, 0)
    }

    /// Example: STA ($A2,X): store the contents of A in the location given in addresses "$00A2+X" and "$00A3+X"
    pub fn store(mem: &mut Memory, pc: LoHi, index: u8, val: u8) {
        let addr = ea(mem, pc, index);

        mem.set(addr, 0, val)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::riot;
        use test_case::test_case;

        #[test_case((0x70,), 0x05, LoHi(0x75, 0x00), LoHi(0x23, 0x30), 0xA5; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        #[test_case((0xFE,), 0x02, LoHi(0x00, 0x00), LoHi(0x23, 0x30), 0xA5; "Page wrap around")]
        fn test_load(op_args: (u8,), index: u8, lookup: LoHi, ea: LoHi, exp: u8) {
            let mut mem = riot::Memory::new(true);
            let pc = 0x0400u16.into();

            // Setup OpCode
            mem.set(pc, 0, 0xA1);
            mem.set(pc, 1, op_args.0);

            // Setup lookup.
            mem.set(lookup, 0, ea.0);
            mem.set(lookup, 1, ea.1);

            // Setup effective address.
            mem.set(ea, 0, exp);

            let obt = super::load(&mem, pc, index);

            assert_eq!(obt, exp);
        }

        #[test_case((0x70,), 0x05, LoHi(0x75, 0x00), LoHi(0x23, 0x30), 0xA5; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        #[test_case((0xFE,), 0x02, LoHi(0x00, 0x00), LoHi(0x23, 0x30), 0xA5; "Page wrap around")]
        fn test_store(op_args: (u8,), index: u8, lookup: LoHi, ea: LoHi, exp: u8) {
            let mut mem = riot::Memory::new(true);
            let pc = 0x0400u16.into();

            // Setup OpCode
            mem.set(pc, 0, 0xA1);
            mem.set(pc, 1, op_args.0);

            // Setup lookup.
            mem.set(lookup, 0, ea.0);
            mem.set(lookup, 1, ea.1);

            super::store(&mut mem, pc, index, 0xA5);

            let obt = mem.get(ea, 0);
            assert_eq!(obt, exp);
        }
    }
}

/// #9 Post-Indexed Indirect, "(Zero-Page),Y"
///
/// LDA ($70),Y - add the contents of the Y-register to the pointer provided in "$0070" and "$0071" and load the contents of this address into A
/// STA ($A2),Y - store the contents of A in the location given by the pointer in "$00A2" and "$00A3" plus the contents of the Y-register
/// EOR ($BA),Y - perform an exlusive OR of the contents of A and the address given by the addition of Y to the pointer in "$00BA" and "$00BB"
pub mod post_indexed_indirect {
    use super::*;

    #[inline]
    fn ea(mem: &Memory, pc: LoHi, index: u8) -> LoHi {
        let lo = indexed_zero_page::load(mem, pc, 0);
        let hi = indexed_zero_page::load(mem, pc, 1);

        LoHi(lo, hi) + index
    }

    pub fn load(mem: &Memory, pc: LoHi, index: u8) -> u8 {
        let addr = ea(mem, pc, index);

        mem.get(addr, 0)
    }

    pub fn store(mem: &mut Memory, pc: LoHi, index: u8, val: u8) {
        let addr = ea(mem, pc, index);

        mem.set(addr, 0, val)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::riot;
        use test_case::test_case;

        #[test_case((0x70,), LoHi(0x70, 0x00), LoHi(0x43, 0x35), 0x10, LoHi(0x53, 0x35), 0x23; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        #[test_case((0x70,), LoHi(0x70, 0x00), LoHi(0xFF, 0x35), 0x01, LoHi(0x00, 0x36), 0x23; "Page wrap around")]
        #[test_case((0x70,), LoHi(0x70, 0x00), LoHi(0xFE, 0xFF), 0x02, LoHi(0x00, 0x00), 0x23; "Address space around")]
        fn test_load(op_args: (u8,), lookup: LoHi, pre_ea: LoHi, index: u8, ea: LoHi, exp: u8) {
            let mut mem = riot::Memory::new(true);
            let pc = 0x0400u16.into();

            // Setup OpCode.
            mem.set(pc, 0, 0xB1);
            mem.set(pc, 1, op_args.0);

            // Setup lookup.
            mem.set(lookup, 0, pre_ea.0);
            mem.set(lookup, 1, pre_ea.1);

            mem.set(ea, 0, exp);

            let obt = super::load(&mem, pc, index);

            assert_eq!(obt, exp);
        }

        #[test_case((0x70,), LoHi(0x70, 0x00), LoHi(0x43, 0x35), 0x10, LoHi(0x53, 0x35), 0x23; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        #[test_case((0x70,), LoHi(0x70, 0x00), LoHi(0xFF, 0x35), 0x01, LoHi(0x00, 0x36), 0x23; "Page wrap around")]
        #[test_case((0x70,), LoHi(0x70, 0x00), LoHi(0xFE, 0xFF), 0x02, LoHi(0x00, 0x00), 0x23; "Address space around")]
        fn test_store(op_args: (u8,), lookup: LoHi, pre_ea: LoHi, index: u8, ea: LoHi, exp: u8) {
            let mut mem = riot::Memory::new(true);
            let pc = 0x0400u16.into();

            // Setup OpCode.
            mem.set(pc, 0, 0xB1);
            mem.set(pc, 1, op_args.0);

            // Setup lookup.
            mem.set(lookup, 0, pre_ea.0);
            mem.set(lookup, 1, pre_ea.1);

            super::store(&mut mem, pc, index, exp);

            let obt = mem.get(ea, 0);
            assert_eq!(obt, exp);
        }
    }
}

/// #10 Relative Addressing (Conditional Branching)
///
/// BEQ $1005 - branch to location "$1005", if the zero flag is set. if the current address is $1000, this will give an offset of $03.
/// BCS $08C4 - branch to location "$08C4", if the carry flag is set. if the current address is $08D4, this will give an offset of $EE (−$12).
/// BCC $084A - branch to location "$084A", if the carry flag is clear.
pub mod relative {
    use super::*;

    /// Refer: https://www.pagetable.com/c64ref/6502/?tab=3#r8
    pub fn load(mem: &Memory, pc: LoHi) -> LoHi {
        let off = mem.get(pc, 1);

        u16::from(pc + 0x02u8) // NOTE: relative is only used in branch opcs, all have length 2
            .wrapping_add_signed(off as i8 as i16)
            .into()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::riot;
        use test_case::test_case;

        #[test_case(LoHi(0x0A, 0xF0), 0xFB, LoHi(0x07, 0xF0); "Jump by -3 bytes")]
        #[test_case(LoHi(0x00, 0x00), 0x50, LoHi(0x52, 0x00); "Jump by 50+len bytes")]
        #[test_case(LoHi(0xFF, 0x44), 0x02, LoHi(0x03, 0x45); "Example from AllSuiteA.bin 0x44FF")]
        #[test_case(LoHi(0x00, 0x10), 0x03, LoHi(0x05, 0x10); "Example 1 from masswerk 6502_instruction_set")]
        #[test_case(LoHi(0xD4, 0x08), 0xEE, LoHi(0xC4, 0x08); "Example 2 from masswerk 6502_instruction_set")]
        #[test_case(LoHi(0x42, 0xF1), 0xFE, LoHi(0x42, 0xF1); "Same instruction")]
        #[test_case(LoHi(0xFF, 0xF1), 0xFE, LoHi(0xFF, 0xF1); "same instruction - wrap across page")]
        #[test_case(LoHi(0x42, 0xF1), 0x7F, LoHi(0xC3, 0xF1); "max front")]
        #[test_case(LoHi(0x35, 0xF0), 0x80, LoHi(0xB7, 0xEF); "max back")]
        #[test_case(LoHi(0xE8, 0xF2), 0x80, LoHi(0x6A, 0xF2); "max back - page wrap")]
        #[test_case(LoHi(0x46, 0xF0), 0x80, LoHi(0xC8, 0xEF); "min")]
        fn test_load(pc: LoHi, op_arg: u8, exp: LoHi) {
            let mut mem = riot::Memory::new(true);
            mem.set(pc, 1, op_arg);

            let obt = super::load(&mem, pc);

            assert_eq!(exp, obt);
        }
    }
}
