use crate::{
    cmn::*,
    cpu::{
        core::{OpcExecutionState, NMOS6502},
        opc_info,
    },
    riot::Memory,
};

/*
    Details:
    - "6502 Address Modes in Detail" in https://www.masswerk.at/6502/6502_instruction_set.html#modes
    - OpCode steps: https://www.nesdev.org/6502_cpu.txt
    - Also refer https://www.pagetable.com/c64ref/6502/?tab=3

    - TODO: There is a whole lot of duplication in this file. Not quiet sure how to remove them using macros.
*/

/// Instructions accessing the stack
pub mod stack {
    /// The break instruction (BRK) behaves like a NMI, but will push the value of PC+2 onto the stack to be used as the return address.
    /// It will also set the I flag. See http://6502.org/tutorials/interrupts.html#2.2.
    /// 0x00 | impl | BRK
    macro_rules! opcode_steps_BRK {
        ($reg_PSR:expr, $illegal:expr) => {
            &[
                // BRK
                //
                //    #  address R/W description
                //   --- ------- --- -----------------------------------------------
                //    1    PC     R  fetch opcode, increment PC
                $illegal,
                //    2    PC     R  read next instruction byte (and throw it away),
                //                   increment PC
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[0] = mem.get(cpu.pc(), 0);
                    cpu.pc_incr(1);
                    false
                },
                //    3  $0100,S  W  push PCH on stack (with B flag set), decrement S
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    mem.set(LoHi(cpu.s(), STACK_POINTER_HI), 0, cpu.pc().1);
                    cpu.set_s(cpu.s().wrapping_sub(1));
                    false
                },
                //    4  $0100,S  W  push PCL on stack, decrement S
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    mem.set(LoHi(cpu.s(), STACK_POINTER_HI), 0, cpu.pc().0);
                    cpu.set_s(cpu.s().wrapping_sub(1));
                    false
                },
                //    5  $0100,S  W  push P on stack, decrement S
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    mem.set(LoHi(cpu.s(), STACK_POINTER_HI), 0, $reg_PSR(cpu));
                    cpu.set_psr_bit(PSR::I);
                    cpu.set_s(cpu.s().wrapping_sub(1));
                    false
                },
                //    6   $FFFE   R  fetch PCL
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    cpu.set_pc(LoHi(mem.get(IRQ_VECTOR, 0), cpu.pc().1));

                    false
                },
                //    7   $FFFF   R  fetch PCH
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    cpu.set_pc(LoHi(cpu.pc().0, mem.get(IRQ_VECTOR, 1)));
                    true
                },
                $illegal,
            ]
        };
    }

    pub(crate) use opcode_steps_BRK;

    macro_rules! opcode_steps_RTI {
        ($set_reg_PSR:expr, $illegal:expr) => {
            &[
                // RTI
                //
                //    #  address R/W description
                //   --- ------- --- -----------------------------------------------
                //    1    PC     R  fetch opcode, increment PC
                $illegal,
                //    2    PC     R  read next instruction byte (and throw it away)
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[0] = mem.get(cpu.pc(), 0);
                    false
                },
                //    3  $0100,S  R  increment S
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, _: &mut Memory| -> bool {
                    cpu.set_s(cpu.s().wrapping_add(1));
                    false
                },
                //    4  $0100,S  R  pull P from stack, increment S
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    let psr = mem.get(LoHi(cpu.s(), STACK_POINTER_HI), 0);
                    $set_reg_PSR(cpu, psr);
                    cpu.set_s(cpu.s().wrapping_add(1));
                    false
                },
                //    5  $0100,S  R  pull PCL from stack, increment S
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    let pc_lo = mem.get(LoHi(cpu.s(), STACK_POINTER_HI), 0);
                    cpu.set_pc(LoHi(pc_lo, cpu.pc().1));
                    cpu.set_s(cpu.s().wrapping_add(1));
                    false
                },
                //    6  $0100,S  R  pull PCH from stack
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    let pc_hi = mem.get(LoHi(cpu.s(), STACK_POINTER_HI), 0);
                    cpu.set_pc(LoHi(cpu.pc().0, pc_hi));
                    true
                },
                $illegal,
                $illegal,
            ]
        };
    }

    pub(crate) use opcode_steps_RTI;

    macro_rules! opcode_steps_RTS {
        ($illegal:expr) => {
            &[
                // RTS
                //
                //    #  address R/W description
                //   --- ------- --- -----------------------------------------------
                //    1    PC     R  fetch opcode, increment PC
                $illegal,
                //    2    PC     R  read next instruction byte (and throw it away)
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[0] = mem.get(cpu.pc(), 0);
                    false
                },
                //    3  $0100,S  R  increment S
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, _: &mut Memory| -> bool {
                    cpu.set_s(cpu.s().wrapping_add(1));
                    false
                },
                //    4  $0100,S  R  pull PCL from stack, increment S
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    let pc_lo = mem.get(LoHi(cpu.s(), STACK_POINTER_HI), 0);
                    cpu.set_pc(LoHi(pc_lo, cpu.pc().1));
                    cpu.set_s(cpu.s().wrapping_add(1));
                    false
                },
                //    5  $0100,S  R  pull PCH from stack
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    let pc_hi = mem.get(LoHi(cpu.s(), STACK_POINTER_HI), 0);
                    cpu.set_pc(LoHi(cpu.pc().0, pc_hi));
                    false
                },
                //    6    PC     R  increment PC
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, _: &mut Memory| -> bool {
                    cpu.pc_incr(1);
                    true
                },
                $illegal,
                $illegal,
            ]
        };
    }

    pub(crate) use opcode_steps_RTS;

    macro_rules! opcode_steps_PHX {
        ($main:expr, $illegal:expr) => {
            &[
                // PHA, PHP
                //
                //    #  address R/W description
                //   --- ------- --- -----------------------------------------------
                //    1    PC     R  fetch opcode, increment PC
                $illegal,
                //    2    PC     R  read next instruction byte (and throw it away)
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[0] = mem.get(cpu.pc(), 0);
                    false
                },
                //    3  $0100,S  W  push register on stack, decrement S
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    mem.set(LoHi(cpu.s(), STACK_POINTER_HI), 0, $main(cpu));
                    cpu.set_s(cpu.s().wrapping_sub(1));
                    true
                },
                $illegal,
                $illegal,
                $illegal,
                $illegal,
                $illegal,
            ]
        };
    }

    pub(crate) use opcode_steps_PHX;

    macro_rules! opcode_steps_PLX {
        ($main:expr, $illegal:expr) => {
            &[
                // PLA, PLP
                //
                //    #  address R/W description
                //   --- ------- --- -----------------------------------------------
                //    1    PC     R  fetch opcode, increment PC
                $illegal,
                //    2    PC     R  read next instruction byte (and throw it away)
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[0] = mem.get(cpu.pc(), 0);
                    false
                },
                //    3  $0100,S  R  increment S
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, _: &mut Memory| -> bool {
                    cpu.set_s(cpu.s().wrapping_add(1));
                    false
                },
                //    4  $0100,S  R  pull register from stack
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    let reg = mem.get(LoHi(cpu.s(), STACK_POINTER_HI), 0);
                    $main(cpu, reg);
                    true
                },
                $illegal,
                $illegal,
                $illegal,
                $illegal,
            ]
        };
    }

    pub(crate) use opcode_steps_PLX;

    macro_rules! opcode_steps_JSR {
        ($illegal:expr) => {
            &[
                // JSR
                //
                //    #  address R/W description
                //   --- ------- --- -------------------------------------------------
                //    1    PC     R  fetch opcode, increment PC
                $illegal,
                //    2    PC     R  fetch low address byte, increment PC
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[0] = mem.get(cpu.pc(), 0);
                    cpu.pc_incr(1);
                    false
                },
                //    3  $0100,S  R  internal operation (predecrement S?)
                |_: &mut OpcExecutionState, _: &mut NMOS6502, _: &mut Memory| -> bool { false },
                //    4  $0100,S  W  push PCH on stack, decrement S
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    mem.set(LoHi(cpu.s(), STACK_POINTER_HI), 0, cpu.pc().1);
                    cpu.set_s(cpu.s().wrapping_sub(1));
                    false
                },
                //    5  $0100,S  W  push PCL on stack, decrement S
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    mem.set(LoHi(cpu.s(), STACK_POINTER_HI), 0, cpu.pc().0);
                    cpu.set_s(cpu.s().wrapping_sub(1));
                    false
                },
                //    6    PC     R  copy low address byte to PCL, fetch high address
                //                   byte to PCH
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    cpu.set_pc(LoHi(s.regs_u8()[0], mem.get(cpu.pc(), 0)));
                    true
                },
                $illegal,
                $illegal,
            ]
        };
    }

    pub(crate) use opcode_steps_JSR;
}

pub mod implied {
    macro_rules! opcode_steps {
        ($main:expr, $illegal:expr) => {
            &[
                // Accumulator or implied addressing
                //
                //       #  address R/W description
                //      --- ------- --- -----------------------------------------------
                //       1    PC     R  fetch opcode, increment PC
                $illegal,
                //       2    PC     R  read next instruction byte (and throw it away)
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[0] = mem.get(cpu.pc(), 0);
                    $main(cpu);
                    true
                },
                $illegal,
                $illegal,
                $illegal,
                $illegal,
                $illegal,
                $illegal,
            ]
        };
    }

    pub(crate) use opcode_steps;
}

/// #2 Immediate Addressing | Immediate
///
/// LDA #$07 - load the literal hexidecimal value "$7" into the accumulator
/// ADC #$A0 - add the literal hexidecimal value "$A0" to the accumulator
/// CPX #$32 - compare the X-register to the literal hexidecimal value "$32"
pub mod immediate {
    use super::*;

    macro_rules! opcode_steps {
        ($main:expr, $illegal:expr) => {
            &[
                // Immediate addressing
                //
                //       #  address R/W description
                //      --- ------- --- ------------------------------------------
                //       1    PC     R  fetch opcode, increment PC
                $illegal,
                //       2    PC     R  fetch value, increment PC
                #[inline]
                |_: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    let val = mem.get(cpu.pc(), 0);
                    cpu.pc_incr(1);
                    $main(cpu, val);
                    true
                },
                $illegal,
                $illegal,
                $illegal,
                $illegal,
                $illegal,
                $illegal,
            ]
        };
    }

    pub(crate) use opcode_steps;

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

    macro_rules! opcode_steps_JMP {
        ($illegal:expr) => {
            &[
                // JMP
                //
                //    #  address R/W description
                //   --- ------- --- -------------------------------------------------
                //    1    PC     R  fetch opcode, increment PC
                $illegal,
                //    2    PC     R  fetch low address byte, increment PC
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[0] = mem.get(cpu.pc(), 0);
                    cpu.pc_incr(1);
                    false
                },
                //    3    PC     R  copy low address byte to PCL, fetch high address
                //                   byte to PCH
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    cpu.set_pc(LoHi(s.regs_u8()[0], mem.get(cpu.pc(), 0)));
                    true
                },
                $illegal,
                $illegal,
                $illegal,
                $illegal,
                $illegal,
            ]
        };
    }

    pub(crate) use opcode_steps_JMP;

    macro_rules! opcode_steps_read {
        ($main:expr, $illegal:expr) => {
            &[
                // Read instructions (LDA, LDX, LDY, EOR, AND, ORA, ADC, SBC, CMP, BIT,
                //                    LAX, NOP)
                //
                //    #  address R/W description
                //   --- ------- --- ------------------------------------------
                //    1    PC     R  fetch opcode, increment PC
                $illegal,
                //    2    PC     R  fetch low byte of address, increment PC
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[0] = mem.get(cpu.pc(), 0);
                    cpu.pc_incr(1);
                    false
                },
                //    3    PC     R  fetch high byte of address, increment PC
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[1] = mem.get(cpu.pc(), 0);
                    cpu.pc_incr(1);
                    false
                },
                //    4  address  R  read from effective address
                //
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    let val = mem.get(LoHi(s.regs_u8()[0], s.regs_u8()[1]), 0);
                    $main(cpu, val);
                    true
                },
                $illegal,
                $illegal,
                $illegal,
                $illegal,
            ]
        };
    }

    pub(crate) use opcode_steps_read;

    macro_rules! opcode_steps_read_modify_write {
        ($main:expr, $illegal:expr) => {
            &[
                // Read-Modify-Write instructions (ASL, LSR, ROL, ROR, INC, DEC,
                //                                 SLO, SRE, RLA, RRA, ISB, DCP)
                //
                //    #  address R/W description
                //   --- ------- --- ------------------------------------------
                //    1    PC     R  fetch opcode, increment PC
                $illegal,
                //    2    PC     R  fetch low byte of address, increment PC
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[0] = mem.get(cpu.pc(), 0);
                    cpu.pc_incr(1);
                    false
                },
                //    3    PC     R  fetch high byte of address, increment PC
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[1] = mem.get(cpu.pc(), 0);
                    cpu.pc_incr(1);
                    false
                },
                //    4  address  R  read from effective address
                |s: &mut OpcExecutionState, _: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[2] = mem.get(LoHi(s.regs_u8()[0], s.regs_u8()[1]), 0);
                    false
                },
                //    5  address  W  write the value back to effective address,
                //                   and do the operation on it
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    mem.set(LoHi(s.regs_u8()[0], s.regs_u8()[1]), 0, s.regs_u8()[2]);
                    s.regs_u8()[2] = $main(cpu, s.regs_u8()[2]);
                    false
                },
                //    6  address  W  write the new value to effective address
                |s: &mut OpcExecutionState, _: &mut NMOS6502, mem: &mut Memory| -> bool {
                    mem.set(LoHi(s.regs_u8()[0], s.regs_u8()[1]), 0, s.regs_u8()[2]);
                    true
                },
                $illegal,
                $illegal,
            ]
        };
    }

    pub(crate) use opcode_steps_read_modify_write;

    macro_rules! opcode_steps_write {
        ($main:expr, $illegal:expr) => {
            &[
                // Write instructions (STA, STX, STY, SAX)
                //
                //    #  address R/W description
                //   --- ------- --- ------------------------------------------
                //    1    PC     R  fetch opcode, increment PC
                $illegal,
                //    2    PC     R  fetch low byte of address, increment PC
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[0] = mem.get(cpu.pc(), 0);
                    cpu.pc_incr(1);
                    false
                },
                //    3    PC     R  fetch high byte of address, increment PC
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[1] = mem.get(cpu.pc(), 0);
                    cpu.pc_incr(1);
                    false
                },
                //    4  address  W  write register to effective address
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    mem.set(LoHi(s.regs_u8()[0], s.regs_u8()[1]), 0, $main(cpu));
                    true
                },
                $illegal,
                $illegal,
                $illegal,
                $illegal,
            ]
        };
    }

    pub(crate) use opcode_steps_write;

    /// Used by JMP & JSR
    #[inline]
    pub fn load_lohi(mem: &Memory, pc: LoHi) -> LoHi {
        LoHi(mem.get(pc, 1), mem.get(pc, 2))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{
            cpu::am::opc_step_illegal,
            cpu::core::{execute_opc_step, OpcExecutionState, MAX_OPCODE_STEPS, NMOS6502},
            riot::Memory,
        };
        use test_case::test_case;

        #[test_case((0x10, 0x30), 0x34; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        fn test_load(op_args: (u8, u8), exp: u8) {
            let mut mem = Memory::new(true);
            let mut cpu = NMOS6502::new(LineState::High.rc_cell(), &mem);
            cpu.set_pc(0x0000u16.into());

            // Setup OpCode.
            mem.set(cpu.pc(), 0, 0xAD);
            mem.set(cpu.pc(), 1, op_args.0);
            mem.set(cpu.pc(), 2, op_args.1);

            // Setup lookup.
            mem.set(op_args.into(), 0, exp);

            cpu.pc_incr(1);
            let steps = opcode_steps_read!(
                |cpu: &mut NMOS6502, val: u8| cpu.set_a(val),
                opc_step_illegal
            );
            for &step in steps.iter().take(MAX_OPCODE_STEPS).skip(1) {
                if execute_opc_step(step, &mut cpu, &mut mem) {
                    break;
                }
            }

            assert_eq!(cpu.a(), exp);
        }

        #[test_case((0x10, 0x30), 0x34; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        fn test_store(op_args: (u8, u8), exp: u8) {
            let mut mem = Memory::new(true);
            let mut cpu = NMOS6502::new(LineState::High.rc_cell(), &mem);
            cpu.set_pc(0x0000u16.into());

            // Setup OpCode.
            mem.set(cpu.pc(), 0, 0xAD);
            mem.set(cpu.pc(), 1, op_args.0);
            mem.set(cpu.pc(), 2, op_args.1);

            cpu.set_a(exp);
            cpu.pc_incr(1);
            let steps = opcode_steps_write!(|cpu: &NMOS6502| cpu.a(), opc_step_illegal);
            for &step in steps.iter().take(MAX_OPCODE_STEPS).skip(1) {
                if execute_opc_step(step, &mut cpu, &mut mem) {
                    break;
                }
            }

            assert_eq!(mem.get(op_args.into(), 0), exp);
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
    macro_rules! opcode_steps_read {
        ($main:expr, $index: expr, $illegal: expr) => {
            &[
                // Read instructions (LDA, LDX, LDY, EOR, AND, ORA, ADC, SBC, CMP, BIT,
                //                    LAX, LAE, SHS, NOP)
                //
                //    #   address  R/W description
                //   --- --------- --- ------------------------------------------
                //    1     PC      R  fetch opcode, increment PC
                $illegal,
                //    2     PC      R  fetch low byte of address, increment PC
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[0] = mem.get(cpu.pc(), 0);
                    cpu.pc_incr(1);
                    false
                },
                //    3     PC      R  fetch high byte of address,
                //                     add index register to low address byte,
                //                     increment PC
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u16()[1] = mem.get(cpu.pc(), 0) as u16;
                    s.regs_u16()[0] = s.regs_u8()[0] as u16 + ($index)(cpu) as u16;
                    cpu.pc_incr(1);
                    false
                },
                //    4  address+I* R  read from effective address,
                //                     fix the high byte of effective address
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    let val = mem.get(LoHi(s.regs_u16()[0] as u8, s.regs_u16()[1] as u8), 0);
                    if crate::bits::tst_bits(s.regs_u16()[0], 0x0100) {
                        s.regs_u16()[1] = s.regs_u16()[1].wrapping_add(1);
                        false
                    } else {
                        $main(cpu, val);
                        true
                    }
                },
                //    5+ address+I  R  re-read from effective address
                //
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    let val = mem.get(LoHi(s.regs_u16()[0] as u8, s.regs_u16()[1] as u8), 0);
                    $main(cpu, val);
                    true
                },
                $illegal,
                $illegal,
                $illegal,
                //   Notes: I denotes either index register (X or Y).
                //
                //          * The high byte of the effective address may be invalid
                //            at this time, i.e. it may be smaller by $100.
                //
                //          + This cycle will be executed only if the effective address
                //            was invalid during cycle #4, i.e. page boundary was crossed.
                //
            ]
        };
    }

    pub(crate) use opcode_steps_read;

    macro_rules! opcode_steps_read_modify_write {
        ($main:expr, $index: expr, $illegal: expr) => {
            &[
                // Read-Modify-Write instructions (ASL, LSR, ROL, ROR, INC, DEC,
                //                                 SLO, SRE, RLA, RRA, ISB, DCP)
                //
                //    #   address  R/W description
                //   --- --------- --- ------------------------------------------
                //    1    PC       R  fetch opcode, increment PC
                $illegal,
                //    2    PC       R  fetch low byte of address, increment PC
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[0] = mem.get(cpu.pc(), 0);
                    cpu.pc_incr(1);
                    false
                },
                //    3    PC       R  fetch high byte of address,
                //                     add index register X to low address byte,
                //                     increment PC
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u16()[1] = mem.get(cpu.pc(), 0) as u16;
                    s.regs_u16()[0] = s.regs_u8()[0] as u16 + ($index)(cpu) as u16;
                    cpu.pc_incr(1);
                    false
                },
                //    4  address+X* R  read from effective address,
                //                     fix the high byte of effective address
                #[inline]
                |s: &mut OpcExecutionState, _: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[1] = mem.get(LoHi(s.regs_u16()[0] as u8, s.regs_u16()[1] as u8), 0);
                    if crate::bits::tst_bits(s.regs_u16()[0], 0x0100) {
                        s.regs_u16()[1] = s.regs_u16()[1].wrapping_add(1);
                    }
                    false
                },
                //    5  address+X  R  re-read from effective address
                #[inline]
                |s: &mut OpcExecutionState, _: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u16()[0] = LoHi(s.regs_u16()[0] as u8, s.regs_u16()[1] as u8).into();
                    s.regs_u8()[1] = mem.get(s.regs_u16()[0].into(), 0);
                    false
                },
                //    6  address+X  W  write the value back to effective address,
                //                     and do the operation on it
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    mem.set(s.regs_u16()[0].into(), 0, s.regs_u8()[1]);
                    s.regs_u8()[1] = $main(cpu, s.regs_u8()[1]);
                    false
                },
                //    7  address+X  W  write the new value to effective address
                #[inline]
                |s: &mut OpcExecutionState, _: &mut NMOS6502, mem: &mut Memory| -> bool {
                    mem.set(s.regs_u16()[0].into(), 0, s.regs_u8()[1]);
                    true
                },
                $illegal,
                //
                //   Notes: * The high byte of the effective address may be invalid
                //            at this time, i.e. it may be smaller by $100.
            ]
        };
    }

    pub(crate) use opcode_steps_read_modify_write;

    macro_rules! opcode_steps_write {
        ($main:expr, $index: expr, $illegal: expr) => {
            &[
                // Write instructions (STA, STX, STY, SHA, SHX, SHY)
                //
                //    #   address  R/W description
                //   --- --------- --- ------------------------------------------
                //    1     PC      R  fetch opcode, increment PC
                $illegal,
                //    2     PC      R  fetch low byte of address, increment PC
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[0] = mem.get(cpu.pc(), 0);
                    cpu.pc_incr(1);
                    false
                },
                //    3     PC      R  fetch high byte of address,
                //                     add index register to low address byte,
                //                     increment PC
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u16()[1] = mem.get(cpu.pc(), 0) as u16;
                    s.regs_u16()[0] = s.regs_u8()[0] as u16 + ($index)(cpu) as u16;
                    cpu.pc_incr(1);
                    false
                },
                //    4  address+I* R  read from effective address,
                //                     fix the high byte of effective address
                #[inline]
                |s: &mut OpcExecutionState, _: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[1] = mem.get(LoHi(s.regs_u16()[0] as u8, s.regs_u16()[1] as u8), 0);
                    if crate::bits::tst_bits(s.regs_u16()[0], 0x0100) {
                        s.regs_u16()[1] = s.regs_u16()[1].wrapping_add(1);
                    }
                    false
                },
                //    5  address+I  W  write to effective address
                //
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    let val = $main(cpu);
                    mem.set(LoHi(s.regs_u16()[0] as u8, s.regs_u16()[1] as u8), 0, val);
                    true
                },
                $illegal,
                $illegal,
                $illegal,
                //   Notes: I denotes either index register (X or Y).
                //
                //          * The high byte of the effective address may be invalid
                //            at this time, i.e. it may be smaller by $100. Because
                //            the processor cannot undo a write to an invalid
                //            address, it always reads from the address first.
            ]
        };
    }

    pub(crate) use opcode_steps_write;

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{
            cmn::{LineState, LoHi, RefExtensions},
            cpu::am::opc_step_illegal,
            cpu::core::{execute_opc_step, OpcExecutionState, MAX_OPCODE_STEPS, NMOS6502},
            riot::Memory,
        };
        use test_case::test_case;

        #[test_case((0x20, 0x31), 0x12, LoHi(0x32, 0x31), 0x78; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        #[test_case((0xFF, 0x31), 0x01, LoHi(0x00, 0x32), 0x78; "Page wrap around")]
        #[test_case((0xFF, 0xFF), 0x01, LoHi(0x00, 0x00), 0x78; "Address space wrap around")]
        fn test_load(op_args: (u8, u8), index: u8, lookup: LoHi, exp: u8) {
            let mut mem = Memory::new(true);
            let mut cpu = NMOS6502::new(LineState::High.rc_cell(), &mem);
            cpu.set_pc(0x0000u16.into());

            // Set up OpCode.
            mem.set(cpu.pc(), 0, 0xBD);
            mem.set(cpu.pc(), 1, op_args.0);
            mem.set(cpu.pc(), 2, op_args.1);

            // Setup lookup.
            mem.set(lookup, 0, exp);

            cpu.set_y(index);
            cpu.pc_incr(1);
            let steps = opcode_steps_read!(
                |cpu: &mut NMOS6502, val: u8| cpu.set_a(val),
                |cpu: &NMOS6502| cpu.y(),
                opc_step_illegal
            );
            for &step in steps.iter().take(MAX_OPCODE_STEPS).skip(1) {
                if execute_opc_step(step, &mut cpu, &mut mem) {
                    break;
                }
            }

            assert_eq!(cpu.a(), exp);
        }

        #[test_case((0x20, 0x31), 0x12, LoHi(0x32, 0x31), 0x78; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        #[test_case((0xFF, 0x31), 0x01, LoHi(0x00, 0x32), 0x78; "Page wrap around")]
        #[test_case((0xFF, 0xFF), 0x01, LoHi(0x00, 0x00), 0x78; "Address space wrap around")]
        fn test_store(op_args: (u8, u8), index: u8, lookup: LoHi, exp: u8) {
            let mut mem = Memory::new(true);
            let mut cpu = NMOS6502::new(LineState::High.rc_cell(), &mem);
            cpu.set_pc(0x0000u16.into());

            // Set up OpCode.
            mem.set(cpu.pc(), 0, 0xBD);
            mem.set(cpu.pc(), 1, op_args.0);
            mem.set(cpu.pc(), 2, op_args.1);

            cpu.set_a(exp);
            cpu.set_y(index);
            cpu.pc_incr(1);
            let steps = opcode_steps_write!(
                |cpu: &NMOS6502| cpu.a(),
                |cpu: &NMOS6502| cpu.y(),
                opc_step_illegal
            );
            for &step in steps.iter().take(MAX_OPCODE_STEPS).skip(1) {
                if execute_opc_step(step, &mut cpu, &mut mem) {
                    break;
                }
            }

            assert_eq!(mem.get(lookup, 0), exp);
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
    pub fn __step2(s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory) -> bool {
        s.regs_u8()[0] = mem.get(cpu.pc(), 0);
        cpu.pc_incr(1);
        false
    }

    #[inline]
    pub fn __step3(s: &mut OpcExecutionState, _: &mut NMOS6502, mem: &mut Memory) -> bool {
        s.regs_u8()[1] = mem.get(LoHi(s.regs_u8()[0], 0x00), 0);
        false
    }

    macro_rules! opcode_steps_read {
        ($main:expr, $index: expr, $illegal: expr) => {
            &[
                // Read instructions (LDA, EOR, AND, ORA, ADC, SBC, CMP)
                //
                //    #    address   R/W description
                //   --- ----------- --- ------------------------------------------
                //    1      PC       R  fetch opcode, increment PC
                $illegal,
                //    2      PC       R  fetch pointer address, increment PC
                crate::cpu::am::post_indexed_indirect::__step2,
                //    3    pointer    R  fetch effective address low
                crate::cpu::am::post_indexed_indirect::__step3,
                //    4   pointer+1   R  fetch effective address high,
                //                       add Y to low byte of effective address
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u16()[1] = mem.get(LoHi(s.regs_u8()[0], 0x00), 1) as u16;
                    s.regs_u16()[0] = s.regs_u8()[1] as u16 + ($index)(cpu) as u16;
                    false
                },
                //    5   address+Y*  R  read from effective address,
                //                       fix high byte of effective address
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    let val = mem.get(LoHi(s.regs_u16()[0] as u8, s.regs_u16()[1] as u8), 0);
                    if crate::bits::tst_bits(s.regs_u16()[0], 0x0100) {
                        s.regs_u16()[1] = s.regs_u16()[1].wrapping_add(1);
                        false
                    } else {
                        $main(cpu, val);
                        true
                    }
                },
                //    6+  address+Y   R  read from effective address
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    let val = mem.get(LoHi(s.regs_u16()[0] as u8, s.regs_u16()[1] as u8), 0);
                    $main(cpu, val);
                    true
                },
                $illegal,
                $illegal,
                //
                //   Notes: The effective address is always fetched from zero page,
                //          i.e. the zero page boundary crossing is not handled.
                //
                //          * The high byte of the effective address may be invalid
                //            at this time, i.e. it may be smaller by $100.
                //
                //          + This cycle will be executed only if the effective address
                //            was invalid during cycle #5, i.e. page boundary was crossed.
                //
            ]
        };
    }

    pub(crate) use opcode_steps_read;

    macro_rules! opcode_steps_write {
        ($main:expr, $index: expr, $illegal: expr) => {
            &[
                // Write instructions (STA, SHA)
                //
                //    #    address   R/W description
                //   --- ----------- --- ------------------------------------------
                //    1      PC       R  fetch opcode, increment PC
                $illegal,
                //    2      PC       R  fetch pointer address, increment PC
                crate::cpu::am::post_indexed_indirect::__step2,
                //    3    pointer    R  fetch effective address low
                crate::cpu::am::post_indexed_indirect::__step3,
                //    4   pointer+1   R  fetch effective address high,
                //                       add Y to low byte of effective address
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u16()[1] = mem.get(LoHi(s.regs_u8()[0], 0x00), 1) as u16;
                    s.regs_u16()[0] = s.regs_u8()[1] as u16 + ($index)(cpu) as u16;
                    false
                },
                //    5   address+Y*  R  read from effective address,
                //                       fix high byte of effective address
                #[inline]
                |s: &mut OpcExecutionState, _: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[2] = mem.get(LoHi(s.regs_u16()[0] as u8, s.regs_u16()[1] as u8), 0);
                    if crate::bits::tst_bits(s.regs_u16()[0], 0x0100) {
                        s.regs_u16()[1] = s.regs_u16()[1].wrapping_add(1);
                    }
                    false
                },
                //    6   address+Y   W  write to effective address
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    let val = $main(cpu);
                    mem.set(LoHi(s.regs_u16()[0] as u8, s.regs_u16()[1] as u8), 0, val);
                    true
                },
                $illegal,
                $illegal,
                //
                //   Notes: The effective address is always fetched from zero page,
                //          i.e. the zero page boundary crossing is not handled.
                //
                //          * The high byte of the effective address may be invalid
                //            at this time, i.e. it may be smaller by $100.
            ]
        };
    }

    pub(crate) use opcode_steps_write;

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{
            cpu::am::opc_step_illegal,
            cpu::core::{execute_opc_step, OpcExecutionState, MAX_OPCODE_STEPS, NMOS6502},
            riot::Memory,
        };
        use test_case::test_case;

        #[test_case((0x70,), LoHi(0x70, 0x00), LoHi(0x43, 0x35), 0x10, LoHi(0x53, 0x35), 0x23; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        #[test_case((0x70,), LoHi(0x70, 0x00), LoHi(0xFF, 0x35), 0x01, LoHi(0x00, 0x36), 0x23; "Page wrap around")]
        #[test_case((0x70,), LoHi(0x70, 0x00), LoHi(0xFE, 0xFF), 0x02, LoHi(0x00, 0x00), 0x23; "Address space around")]
        fn test_load(op_args: (u8,), lookup: LoHi, pre_ea: LoHi, index: u8, ea: LoHi, exp: u8) {
            let mut mem = Memory::new(true);
            let mut cpu = NMOS6502::new(LineState::High.rc_cell(), &mem);
            cpu.set_pc(0x0400u16.into());

            // Setup OpCode.
            mem.set(cpu.pc(), 0, 0xB1);
            mem.set(cpu.pc(), 1, op_args.0);

            // Setup lookup.
            mem.set(lookup, 0, pre_ea.0);
            mem.set(lookup, 1, pre_ea.1);

            mem.set(ea, 0, exp);

            cpu.set_y(index);
            cpu.pc_incr(1);
            let steps = opcode_steps_read!(
                |cpu: &mut NMOS6502, val: u8| cpu.set_a(val),
                |cpu: &NMOS6502| cpu.y(),
                opc_step_illegal
            );
            for &step in steps.iter().take(MAX_OPCODE_STEPS).skip(1) {
                if execute_opc_step(step, &mut cpu, &mut mem) {
                    break;
                }
            }

            assert_eq!(cpu.a(), exp);
        }

        #[test_case((0x70,), LoHi(0x70, 0x00), LoHi(0x43, 0x35), 0x10, LoHi(0x53, 0x35), 0x23; "Example from https://www.masswerk.at/6502/6502_instruction_set.htm")]
        #[test_case((0x70,), LoHi(0x70, 0x00), LoHi(0xFF, 0x35), 0x01, LoHi(0x00, 0x36), 0x23; "Page wrap around")]
        #[test_case((0x70,), LoHi(0x70, 0x00), LoHi(0xFE, 0xFF), 0x02, LoHi(0x00, 0x00), 0x23; "Address space around")]
        fn test_store(op_args: (u8,), lookup: LoHi, pre_ea: LoHi, index: u8, ea: LoHi, exp: u8) {
            let mut mem = Memory::new(true);
            let mut cpu = NMOS6502::new(LineState::High.rc_cell(), &mem);
            cpu.set_pc(0x0400u16.into());

            // Setup OpCode.
            mem.set(cpu.pc(), 0, 0xB1);
            mem.set(cpu.pc(), 1, op_args.0);

            // Setup lookup.
            mem.set(lookup, 0, pre_ea.0);
            mem.set(lookup, 1, pre_ea.1);

            cpu.set_a(exp);
            cpu.set_y(index);
            cpu.pc_incr(1);
            let steps = opcode_steps_write!(
                |cpu: &NMOS6502| cpu.a(),
                |cpu: &NMOS6502| cpu.y(),
                opc_step_illegal
            );
            for &step in steps.iter().take(MAX_OPCODE_STEPS).skip(1) {
                if execute_opc_step(step, &mut cpu, &mut mem) {
                    break;
                }
            }

            assert_eq!(mem.get(ea, 0), exp);
        }
    }
}

/// #10 Relative Addressing (Conditional Branching)
///
/// BEQ $1005 - branch to location "$1005", if the zero flag is set. if the current address is $1000, this will give an offset of $03.
/// BCS $08C4 - branch to location "$08C4", if the carry flag is set. if the current address is $08D4, this will give an offset of $EE (−$12).
/// BCC $084A - branch to location "$084A", if the carry flag is clear.
///
pub mod relative {
    macro_rules! opcode_steps {
        ($main:expr, $illegal:expr) => {
            &[
                //  Relative addressing (BCC, BCS, BNE, BEQ, BPL, BMI, BVC, BVS)
                //
                //        #   address  R/W description
                //       --- --------- --- ---------------------------------------------
                //        1     PC      R  fetch opcode, increment PC
                $illegal,
                //        2     PC      R  fetch operand, increment PC
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[0] = mem.get(cpu.pc(), 0);
                    cpu.pc_incr(1);
                    !$main(cpu)
                },
                //        3     PC      R  Fetch opcode of next instruction,
                //                         If branch is taken, add operand to PCL.
                //                         Otherwise increment PC.
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[3] = mem.get(cpu.pc(), 0);
                    let new_pc =
                        u16::from(cpu.pc()).wrapping_add_signed(s.regs_u8()[0] as i8 as i16);
                    cpu.set_pc(LoHi(new_pc as u8, cpu.pc().1));
                    s.regs_u8()[1] = (new_pc >> 8) as u8;
                    cpu.pc().1 == s.regs_u8()[1]
                },
                //        4+    PC*     R  Fetch opcode of next instruction.
                //                         Fix PCH. If it did not change, increment PC.
                #[inline]
                |s: &mut OpcExecutionState, cpu: &mut NMOS6502, mem: &mut Memory| -> bool {
                    s.regs_u8()[3] = mem.get(cpu.pc(), 0);
                    let new_pc = LoHi(cpu.pc().0, s.regs_u8()[1]);
                    cpu.set_pc(new_pc);
                    true
                },
                $illegal,
                $illegal,
                $illegal,
                $illegal,
                //        5!    PC      R  Fetch opcode of next instruction,
                //                         increment PC.
                //
                //       Notes: The opcode fetch of the next instruction is included to
                //              this diagram for illustration purposes. When determining
                //              real execution times, remember to subtract the last
                //              cycle.
                //
                //              * The high byte of Program Counter (PCH) may be invalid
                //                at this time, i.e. it may be smaller or bigger by $100.
                //
                //              + If branch is taken, this cycle will be executed.
                //
                //              ! If branch occurs to different page, this cycle will be
                //                executed.
            ]
        };
    }

    pub(crate) use opcode_steps;

    #[cfg(test)]
    mod tests {
        use crate::{
            cmn::*,
            cpu::am::opc_step_illegal,
            cpu::core::{execute_opc_step, OpcExecutionState, MAX_OPCODE_STEPS, NMOS6502},
            riot::Memory,
        };
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
            let mut mem = Memory::new(true);
            let mut cpu = NMOS6502::new(LineState::High.rc_cell(), &mem);
            cpu.set_pc(pc);
            mem.set(pc, 1, op_arg);
            cpu.pc_incr(1);

            let steps = opcode_steps!(|_| true, opc_step_illegal);

            for &step in steps.iter().take(MAX_OPCODE_STEPS).skip(1) {
                if execute_opc_step(step, &mut cpu, &mut mem) {
                    break;
                }
            }

            assert_eq!(cpu.pc(), exp);
        }
    }
}

pub fn opc_step_illegal(s: &mut OpcExecutionState, cpu: &mut NMOS6502, _: &mut Memory) -> bool {
    let opc_info = &opc_info::ALL[s.opc()];
    unimplemented!(
        "Step #{} for Opcode {:02X} ({}) not implemented. CPU state: {cpu:?}",
        s.step(),
        s.opc(),
        opc_info.assembler
    )
}

macro_rules! stub_opcode_steps {
    ($illegal:expr) => {
        &[
            $illegal, $illegal, $illegal, $illegal, $illegal, $illegal, $illegal, $illegal,
        ]
    };
}

pub(crate) use stub_opcode_steps;
