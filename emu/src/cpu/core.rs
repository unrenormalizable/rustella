use crate::bits;
use crate::{
    cmn::*,
    cpu::{cmn, opc_impl, opc_info, timer},
    mem::Memory,
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
#[derive(Default)]
/// Refer: https://www.princeton.edu/~mae412/HANDOUTS/Datasheets/6502.pdf
pub struct MOS6502 {
    A: u8,
    X: u8,
    Y: u8,
    PC: LoHi,
    S: u8,
    P: PSR,
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
pub type OpCode = dyn Fn(&mut MOS6502, &mut Memory, u8, LoHi) -> Option<LoHi>;

impl MOS6502 {
    pub fn new(mem: &Memory) -> Self {
        let mut cpu = Self::default();
        cpu.reset_pc(mem);

        cpu
    }

    #[inline]
    pub fn tick(&mut self, mem: &mut Memory) -> usize {
        let start_time = timer::get_nanoseconds();
        let opc = mem.get(self.PC, 0);
        let res = opc_impl::ALL_OPCODE_ROUTINES[opc as usize](self, mem, opc, self.PC);
        if let Some(lohi) = res {
            self.PC = lohi;
        } else {
            self.pc_incr(opc_info::ALL[opc as usize].bytes);
        }

        let cycles = opc_info::ALL[opc as usize].cycles;
        self.instructions += 1;
        self.cycles += cycles;
        self.duration += timer::measure_elapsed(start_time);

        cycles
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

    pub fn instructions(&self) -> u64 {
        self.instructions
    }

    pub fn cycles(&self) -> usize {
        self.cycles
    }

    pub fn duration(&self) -> u64 {
        let overhead = timer::measure_overhead();
        self.duration.saturating_sub(self.instructions * overhead)
    }

    #[inline]
    fn pc_incr(&mut self, index: u8) {
        self.PC += index;
    }

    fn reset_pc(&mut self, mem: &Memory) {
        let pc_lo = mem.get(cmn::RST_VECTOR, 0);
        let pc_hi = mem.get(cmn::RST_VECTOR, 1);

        self.PC = LoHi(pc_lo, pc_hi);
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
