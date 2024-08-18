use super::{hw_dbg, mem, opcode};
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

        /// Break command.
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
        PSR::N | PSR::V | PSR::B | PSR::I | PSR::Z | PSR::C
    }
}

#[allow(non_snake_case)]
/// Refer: https://www.princeton.edu/~mae412/HANDOUTS/Datasheets/6502.pdf
pub struct MOS6502 {
    A: u8,
    Y: u8,
    X: u8,
    PC_lo: u8,
    PC_hi: u8,
    S: u8,
    P: PSR,
}

type HwDebuggerCallback = fn(opc: u8, cpu: &mut MOS6502, mem: &mut mem::Memory) -> (bool, usize);

impl MOS6502 {
    pub fn new(pc_lo: u8, pc_hi: u8) -> Self {
        Self {
            A: 0xde,
            Y: 0xad,
            X: 0xbe,
            PC_lo: pc_lo,
            PC_hi: pc_hi,
            S: 0xef,
            P: PSR::default(),
        }
    }

    /// References:
    /// - Patterns: https://llx.com/Neil/a2/opcodes.html
    /// - Instruction set: https://www.masswerk.at/6502/6502_instruction_set.html
    ///
    /// TODO: Remove the callback once we find a better signalling mechanism to indicate hw breakpoint.
    pub fn fetch_decode_execute(&mut self, mem: &mut mem::Memory, callback: HwDebuggerCallback) {
        let mut call_dbg_after = 0;
        loop {
            let opc = mem.get(self.PC_lo, self.PC_hi, 0);
            if call_dbg_after == 0 {
                let cb_res = callback(opc, self, mem);
                call_dbg_after = cb_res.1;
                if !cb_res.0 {
                    break;
                }
            }

            let res =
                opcode::ALL_OPCODE_ROUTINES[opc as usize](self, mem, opc, self.PC_lo, self.PC_hi);
            if let Some((lo, hi)) = res {
                self.PC_lo = lo;
                self.PC_hi = hi;
            } else {
                self.pc_incr(hw_dbg::ALL_OPCODE_INFO[opc as usize].bytes);
            }
            call_dbg_after -= 1;
        }
    }

    pub fn tst_psr_bit(&mut self, bit: PSR) -> bool {
        tst_bit(self.P.bits(), bit.bits())
    }

    pub fn set_psr_bit(&mut self, bit: PSR) {
        set_bit(&mut self.P, bit);
    }

    pub fn clr_psr_bit(&mut self, bit: PSR) {
        clr_bit(&mut self.P, bit)
    }

    pub fn a(&self) -> u8 {
        self.A
    }

    pub fn set_a(&mut self, a: u8) {
        self.A = a;
    }

    pub fn x(&self) -> u8 {
        self.X
    }

    pub fn set_x(&mut self, x: u8) {
        self.X = x;
    }

    pub fn y(&self) -> u8 {
        self.Y
    }

    pub fn set_y(&mut self, y: u8) {
        self.Y = y;
    }

    pub fn s(&self) -> u8 {
        self.S
    }

    pub fn set_s(&mut self, s: u8) {
        self.S = s;
    }

    pub fn p(&self) -> u8 {
        self.P.bits()
    }

    pub fn set_p(&mut self, p: u8) {
        self.P = PSR::from_bits_truncate(p);
    }

    pub fn pc(&self) -> (u8, u8) {
        (self.PC_lo, self.PC_hi)
    }

    fn pc_incr(&mut self, incr: u8) {
        // TODO: safe add
        self.PC_lo += incr;
    }
}

pub fn tst_bit(bits: u8, bit: u8) -> bool {
    bits & bit == bit
}

fn set_bit(bits: &mut PSR, bit: PSR) {
    *bits |= bit;
}

fn clr_bit(bits: &mut PSR, bit: PSR) {
    *bits &= !bit;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tst_bit() {
        let bits: PSR = PSR::B | PSR::C;
        assert!(tst_bit(bits.bits(), PSR::B.bits()));
        assert!(!tst_bit(bits.bits(), PSR::V.bits()));
    }

    #[test]
    fn test_set_bit() {
        let mut bits: PSR = !PSR::from_bits_truncate(0);
        set_bit(&mut bits, PSR::B);

        assert!(tst_bit(bits.bits(), PSR::B.bits()));
    }

    #[test]
    fn test_clr_bit() {
        let mut bits: PSR = PSR::from_bits_truncate(0);
        clr_bit(&mut bits, PSR::B);

        assert!(!tst_bit(bits.bits(), PSR::B.bits()));
    }
}
