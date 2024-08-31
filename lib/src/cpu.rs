use super::{cmn::*, mem, opc_impl, opc_info};
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
    PC: LoHi,
    S: u8,
    P: PSR,
}

pub type HwDebuggerCallback =
    fn(opc: u8, cpu: &mut MOS6502, mem: &mut mem::Memory) -> (bool, usize);

/// References (use multiple to cross check implementation):
/// - https://www.masswerk.at/6502/6502_instruction_set.html
/// - https://www.pagetable.com/c64ref/6502/
pub type OpCode = dyn Fn(&mut MOS6502, &mut mem::Memory, u8, LoHi) -> Option<LoHi>;

impl Default for MOS6502 {
    fn default() -> Self {
        Self {
            A: 0xde,
            Y: 0xad,
            X: 0xbe,
            PC: RESET_VECTOR,
            S: 0xef,
            P: PSR::default(),
        }
    }
}

impl MOS6502 {
    /// References:
    /// - Patterns: https://llx.com/Neil/a2/opcodes.html
    /// - Instruction set: https://www.masswerk.at/6502/6502_instruction_set.html
    ///
    /// TODO: Remove the callback once we find a better signalling mechanism to indicate hw breakpoint.
    pub fn fetch_decode_execute(&mut self, mem: &mut mem::Memory, callback: HwDebuggerCallback) {
        let mut call_dbg_after = 0;

        self.set_pc(Self::get_pc_from_reset_vector(mem));
        loop {
            let opc = mem.get(self.PC, 0);
            if call_dbg_after == 0 {
                let cb_res = callback(opc, self, mem);
                call_dbg_after = cb_res.1;
                if !cb_res.0 {
                    break;
                }
            }

            self.exec_one_opcode(mem, opc);
            call_dbg_after -= 1;
        }
    }

    pub fn tst_psr_bit(&self, bit: PSR) -> bool {
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

    pub fn pc(&self) -> LoHi {
        self.PC
    }

    pub fn set_pc(&mut self, val: LoHi) {
        self.PC = val;
    }

    fn pc_incr(&mut self, index: u8) {
        self.PC += index;
    }

    fn exec_one_opcode(&mut self, mem: &mut mem::Memory, opc: u8) {
        let res = opc_impl::ALL_OPCODE_ROUTINES[opc as usize](self, mem, opc, self.PC);
        if let Some(lohi) = res {
            self.PC = lohi;
        } else {
            self.pc_incr(opc_info::ALL[opc as usize].bytes);
        }
    }

    fn get_pc_from_reset_vector(mem: &mem::Memory) -> LoHi {
        let pc_lo = mem.get(RESET_VECTOR, 0);
        let pc_hi = mem.get(RESET_VECTOR, 1);

        LoHi(pc_lo, pc_hi)
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
