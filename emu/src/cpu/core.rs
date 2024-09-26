use crate::{bits, timer};
use crate::{
    cmn::*,
    cpu::{cmn, opc_impl::*, opc_info},
    riot::Memory,
};
use bitflags::bitflags;

bitflags! {
    pub struct PSR: u8 {
        /// Carry.
        const C = 1 << 0;

        /// Zero.
        const Z = 1 << 1;

        /// Interrupt Disable.
        const I = 1 << 2;

        /// Decimal
        const D = 1 << 3;

        /// Break flag
        /// - Is not accessed by the CPU at anytime and there is no internal representation.
        /// - Will be inserted when PSR is transferred to the stack by software (BRK or PHP), and will be zero, when transferred by hardware.
        /// - Will be ignored when retrieved by software (PLP or RTI).
        const B = 1 << 4;

        /// Ignored.
        const __ = 1 << 5;

        /// Overflow.
        const V = 1 << 6;

        /// Negative.
        const N = 1 << 7;
    }
}

impl Default for PSR {
    fn default() -> Self {
        PSR::I
    }
}

#[allow(non_snake_case)]
#[derive(Default, Clone)]
/// Refer: https://www.princeton.edu/~mae412/HANDOUTS/Datasheets/6502.pdf
pub struct MOS6502 {
    A: u8,
    X: u8,
    Y: u8,
    PC: LoHi,
    S: u8,
    P: PSR,
    // Other pins.
    rdy: Line,
    // Clock cycle bookkeeping
    execution_state: ExecutionState,
    // Profiling stuff, maybe move them elsewhere?
    instructions: u64,
    cycles: usize,
    duration: u64,
}

impl core::fmt::Debug for MOS6502 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f,
            "CPU: A: {:02X}, X: {:02X}, Y: {:02X}, PC: {:?}, S: {:02X}, P: {:?}\nMetrics: instructions: {}, cycles: {}, duration: {}",
            self.A, self.X, self.Y, self.PC, self.S, self.P, self.instructions, self.cycles, self.duration)
    }
}

/// References (use multiple to cross check implementation):
/// - https://www.masswerk.at/6502/6502_instruction_set.html
/// - https://www.pagetable.com/c64ref/6502/
pub type OpCodeFn = dyn Fn(&mut MOS6502, &mut Memory) -> Option<LoHi>;

pub type OpCodeStepFn = dyn Fn(&mut MOS6502, &mut Memory) -> bool;

impl MOS6502 {
    pub fn new(rdy: Line, mem: &Memory) -> Self {
        let mut cpu = Self {
            rdy,
            execution_state: ExecutionState {
                done: true,
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.reset_pc(mem);

        cpu
    }

    pub fn reset_pc(&mut self, mem: &Memory) {
        let pc_lo = mem.get(cmn::RST_VECTOR, 0);
        let pc_hi = mem.get(cmn::RST_VECTOR, 1);

        self.PC = LoHi(pc_lo, pc_hi);
    }

    /// Refer: https://www.nesdev.org/6502_cpu.txt
    #[inline]
    pub fn tick(&mut self, mem: &mut Memory) -> usize {
        if let LineState::Low = self.rdy.get() {
            return 0;
        }

        let start_time = timer::get_nanoseconds();
        let opc = mem.get(self.PC, 0) as usize;
        // Clock cycle inaccurate code path.
        if self.execution_state.done && !NEW_CODE_PATH[opc] {
            let res = ALL_OPCODE_ROUTINES[opc](self, mem);
            if let Some(lohi) = res {
                self.PC = lohi;
            } else {
                self.pc_incr(opc_info::ALL[opc].bytes);
            }

            let cycles = opc_info::ALL[opc].cycles;
            self.instructions += 1;
            self.cycles += cycles;
            self.duration += timer::measure_elapsed(start_time);

            return cycles;
        }

        // Clock cycle accurate code path.
        self.execution_state.done = if self.execution_state.done {
            // This is the same Step 0 for all opcodes.
            let opc = mem.get(self.PC, 0) as usize;
            self.pc_incr(1);

            self.execution_state.opc = opc;
            self.execution_state.step = 0;

            false
        } else {
            ALL_OPCODE_STEPS[self.execution_state.opc][self.execution_state.step](self, mem)
        };

        self.execution_state.step += 1;
        if self.execution_state.done {
            self.instructions += 1;
        }
        self.cycles += 1;
        self.duration += timer::measure_elapsed(start_time);
        1
    }

    #[inline]
    pub fn tst_psr_bit(&self, bit: PSR) -> bool {
        bits::tst_bits(self.P.bits(), bit.bits())
    }

    #[inline]
    pub fn set_psr_bit(&mut self, bit: PSR) {
        set_bit(&mut self.P, bit);
    }

    #[inline]
    pub fn clr_psr_bit(&mut self, bit: PSR) {
        clr_bit(&mut self.P, bit)
    }

    #[inline]
    pub fn a(&self) -> u8 {
        self.A
    }

    #[inline]
    pub fn set_a(&mut self, a: u8) {
        self.A = a;
    }

    #[inline]
    pub fn x(&self) -> u8 {
        self.X
    }

    #[inline]
    pub fn set_x(&mut self, x: u8) {
        self.X = x;
    }

    #[inline]
    pub fn y(&self) -> u8 {
        self.Y
    }

    #[inline]
    pub fn set_y(&mut self, y: u8) {
        self.Y = y;
    }

    #[inline]
    pub fn s(&self) -> u8 {
        self.S
    }

    #[inline]
    pub fn set_s(&mut self, s: u8) {
        self.S = s;
    }

    #[inline]
    pub fn psr(&self) -> u8 {
        self.P.bits()
    }

    #[inline]
    pub fn set_psr(&mut self, p: u8) {
        self.P = PSR::from_bits_truncate(p);
    }

    #[inline]
    pub fn pc(&self) -> LoHi {
        self.PC
    }

    #[inline]
    pub fn set_pc(&mut self, val: LoHi) {
        self.PC = val;
    }

    #[inline]
    pub fn execution_state(&mut self) -> &mut ExecutionState {
        &mut self.execution_state
    }

    #[inline]
    pub fn instructions(&self) -> u64 {
        self.instructions
    }

    #[inline]
    pub fn cycles(&self) -> usize {
        self.cycles
    }

    #[inline]
    pub fn duration(&self) -> u64 {
        let overhead = timer::measure_overhead();
        self.duration.saturating_sub(self.instructions * overhead)
    }

    #[inline]
    pub fn pc_incr(&mut self, index: u8) {
        self.PC += index;
    }
}

#[derive(Default, Clone)]
pub struct ExecutionState {
    opc: usize,
    done: bool,
    step: usize,
    throw_away: u8,
    regs: [u8; 2],
}

impl ExecutionState {
    #[inline]
    pub fn opc(&self) -> usize {
        self.opc
    }

    #[inline]
    pub fn step(&self) -> usize {
        self.step
    }

    #[inline]
    pub fn regs(&mut self) -> &mut [u8; 2] {
        &mut self.regs
    }

    #[inline]
    pub fn set_throw_away(&mut self, val: u8) {
        self.throw_away = val
    }
}

#[inline]
fn set_bit(bits: &mut PSR, bit: PSR) {
    *bits |= bit;
}

#[inline]
fn clr_bit(bits: &mut PSR, bit: PSR) {
    *bits &= !bit;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bits;

    #[test]
    fn test_tst_bit() {
        let bits: PSR = PSR::B | PSR::C;
        assert!(bits::tst_bits(bits.bits(), PSR::B.bits()));
        assert!(!bits::tst_bits(bits.bits(), PSR::V.bits()));
    }

    #[test]
    fn test_set_bit() {
        let mut bits: PSR = !PSR::from_bits_truncate(0);
        set_bit(&mut bits, PSR::B);

        assert!(bits::tst_bits(bits.bits(), PSR::B.bits()));
    }

    #[test]
    fn test_clr_bit() {
        let mut bits: PSR = PSR::from_bits_truncate(0);
        clr_bit(&mut bits, PSR::B);

        assert!(!bits::tst_bits(bits.bits(), PSR::B.bits()));
    }
}
