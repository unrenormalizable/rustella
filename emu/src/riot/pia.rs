use crate::{bits, riot::cmn::*};

/// Refer:
/// - https://www.alienbill.com/2600/101/docs/stella.html#pia1.0
pub trait PIA6532: MemorySegment {
    fn tick(&mut self, cycles: usize);
}

#[derive(Default)]
pub struct InMemory6532 {
    timer_clk: usize,
    timer_count: u8,
    timer_factor: u16,
}

impl PIA6532 for InMemory6532 {
    fn tick(&mut self, cycles: usize) {
        for _ in 0..cycles {
            self.one_tick();
        }
    }
}

impl MemorySegment for InMemory6532 {
    fn read(&self, addr: usize) -> u8 {
        #[cfg(debug_assertions)]
        self.check_read_unsupported_register_flags(addr);

        if addr == regs::INTIM {
            return self.timer_count;
        }

        if addr == regs::SWCHB {
            return bits::BIT_D3;
        }

        0
    }

    fn write(&mut self, addr: usize, val: u8) {
        let (_, _, name, supported_write_mask) =
            regs::IMPLEMENTED_REGISTERS[addr - IOT_MIN_ADDRESS];

        #[cfg(debug_assertions)]
        self.check_write_unsupported_register_flags(addr, val);

        (self.timer_count, self.timer_factor) = match addr {
            regs::TIM1T => (val, 1),
            regs::TIM8T => (val, 8),
            regs::TIM64T => (val, 64),
            regs::T1024T => (val, 1024),
            _ => panic!("Should have failed in check_write_unsupported_register_flags. Reg {name}, supported bits: {supported_write_mask:08b}, val: {val:08b}."),
        };
        self.timer_clk = self.timer_count as usize * self.timer_factor as usize;
    }
}

impl InMemory6532 {
    fn one_tick(&mut self) {
        if self.timer_clk != 0 {
            self.timer_clk -= 1;
            self.timer_count = self.timer_clk.div_ceil(self.timer_factor as usize) as u8;
        }
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn check_read_unsupported_register_flags(&self, addr: usize) {
        let (r, _, name, _) = regs::IMPLEMENTED_REGISTERS[addr - IOT_MIN_ADDRESS];

        assert!(r, "Reading {name} ({addr:02X}) is not implemented yet.")
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn check_write_unsupported_register_flags(&self, addr: usize, val: u8) {
        let (_, w, name, supported_write_mask) =
            regs::IMPLEMENTED_REGISTERS[addr - IOT_MIN_ADDRESS];

        assert!(
            w || val == 0x00, // NOTE: CLEAN_START macro sets everything to 0x00.
            "{name} ({addr:02X}) is not implemented yet, Value 0b{val:08b}."
        );

        assert!(
            val & !supported_write_mask == 0,
            "{name} ({addr:02X}) for value 0b{val:08b} is not implemented yet. Supported bits 0b{supported_write_mask:08b}."
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn simple_timer_test() {
        let mut pia = InMemory6532::default();

        pia.write(regs::TIM8T, 1);
        assert_eq!(pia.read(regs::INTIM), 1);

        pia.tick(1);
        assert_eq!(pia.read(regs::INTIM), 1);
        pia.tick(1);
        assert_eq!(pia.read(regs::INTIM), 1);
        pia.tick(1);
        assert_eq!(pia.read(regs::INTIM), 1);
        pia.tick(1);
        assert_eq!(pia.read(regs::INTIM), 1);
        pia.tick(1);
        assert_eq!(pia.read(regs::INTIM), 1);
        pia.tick(1);
        assert_eq!(pia.read(regs::INTIM), 1);
        pia.tick(1);
        assert_eq!(pia.read(regs::INTIM), 1);
        pia.tick(1);
        assert_eq!(pia.read(regs::INTIM), 0);
        pia.tick(1);
        assert_eq!(pia.read(regs::INTIM), 0);
        pia.tick(1);
        assert_eq!(pia.read(regs::INTIM), 0);
        pia.tick(1);
        assert_eq!(pia.read(regs::INTIM), 0);
    }

    #[test_case(regs::TIM1T, 2, 0, 2; "TIM1T 0 cycles")]
    #[test_case(regs::TIM1T, 2, 1, 1; "TIM1T less cycles")]
    #[test_case(regs::TIM1T, 2, 2, 0; "TIM1T same cycles")]
    #[test_case(regs::TIM1T, 2, 3, 0; "TIM1T more cycles")]
    #[test_case(regs::TIM8T, 2, 0, 2; "TIM8T 0 cycles")]
    #[test_case(regs::TIM8T, 2, 7, 2; "TIM8T less cycles - non multiple")]
    #[test_case(regs::TIM8T, 2, 8, 1; "TIM8T less cycles")]
    #[test_case(regs::TIM8T, 2, 16, 0; "TIM8T same cycles")]
    #[test_case(regs::TIM8T, 2, 17, 0; "TIM8T more cycles")]
    #[test_case(regs::TIM64T, 2, 0, 2; "TIM64T 0 cycles")]
    #[test_case(regs::TIM64T, 2, 63, 2; "TIM64T less cycles - non multiple")]
    #[test_case(regs::TIM64T, 2, 64, 1; "TIM64T less cycles")]
    #[test_case(regs::TIM64T, 2, 65, 1; "TIM64T less cycles - non multiple - 2")]
    #[test_case(regs::TIM64T, 2, 128, 0; "TIM64T same cycles")]
    #[test_case(regs::TIM64T, 2, 129, 0; "TIM64T more cycles")]
    #[test_case(regs::T1024T, 2, 0, 2; "T1024T 0 cycles")]
    #[test_case(regs::T1024T, 2, 1020, 2; "T1024T less cycles - non multiple")]
    #[test_case(regs::T1024T, 2, 1024, 1; "T1024T less cycles")]
    #[test_case(regs::T1024T, 2, 1030, 1; "T1024T less cycles - non multiple - 2")]
    #[test_case(regs::T1024T, 2, 2048, 0; "T1024T same cycles")]
    #[test_case(regs::T1024T, 2, 3000, 0; "T1024T more cycles")]
    fn timer_tests(reg: usize, val: u8, ticks: usize, remain: u8) {
        let mut pia = InMemory6532::default();

        pia.write(reg, val);

        pia.tick(ticks);

        assert_eq!(pia.read(regs::INTIM), remain);
    }
}
