use super::{mem, opcode};
use bitflags::bitflags;

bitflags! {
    pub struct PSR: u8 {
        const C = 1 << 0;
        const Z = 1 << 1;
        const I = 1 << 2;
        const D = 1 << 3;
        const B = 1 << 4;
        const V = 1 << 6;
        const N = 1 << 7;

        const ALL = 0xff;
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
/// Refer: https://www.princeton.edu/~mae412/HANDOUTS/Datasheets/6502.pdf
pub struct MCS6502 {
    A: u8,
    Y: u8,
    X: u8,
    PC_lo: u8,
    PC_hi: u8,
    S: u8,
    P: PSR,
}

impl MCS6502 {
    pub fn new() -> Self {
        Self {
            A: 0xde,
            Y: 0xad,
            X: 0xbe,
            PC_lo: 0x00,
            PC_hi: 0x00,
            S: 0xef,
            P: !PSR::ALL,
        }
    }

    /// References:
    /// - Patterns: https://llx.com/Neil/a2/opcodes.html
    /// - Instruction set: https://www.masswerk.at/6502/6502_instruction_set.html
    pub fn fetch_decode_execute(&mut self, mem: &mut mem::Memory, init_pc_lo: u8, init_pc_hi: u8) {
        self.PC_lo = init_pc_lo;
        self.PC_hi = init_pc_hi;

        loop {
            let opc = mem.get(self.PC_lo, self.PC_hi);
            let opc = opcode::ALL_OPCODES[opc as usize];
            let pc = opc(self, mem);

            self.PC_lo = pc.0;
            self.PC_hi = pc.1;
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

    pub fn x(&mut self) -> u8 {
        self.X
    }

    pub fn set_x(&mut self, x: u8) {
        self.X = x;
    }

    pub fn set_s(&mut self, s: u8) {
        self.S = s;
    }

    pub fn pc(&self, off_lo: u8) -> (u8, u8) {
        (self.PC_lo + off_lo, self.PC_hi)
    }
}

impl Default for MCS6502 {
    fn default() -> Self {
        Self::new()
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
        let mut bits: PSR = !PSR::ALL;
        set_bit(&mut bits, PSR::B);

        assert!(tst_bit(bits.bits(), PSR::B.bits()));
    }

    #[test]
    fn test_clr_bit() {
        let mut bits: PSR = PSR::ALL;
        clr_bit(&mut bits, PSR::B);

        assert!(!tst_bit(bits.bits(), PSR::B.bits()));
    }
}
