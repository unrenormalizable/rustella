use crate::riot::MemorySegment;

pub const IOT_MIN_ADDRESS: usize = 0x0280;
pub const IOT_MAX_ADDRESS: usize = 0x029F;

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

        0
    }

    fn write(&mut self, addr: usize, val: u8) {
        #[cfg(debug_assertions)]
        self.check_write_unsupported_register_flags(addr, val);

        (self.timer_count, self.timer_factor) = match addr {
            regs::TIM1T => (val, 1),
            regs::TIM8T => (val, 8),
            regs::TIM64T => (val, 64),
            regs::T1024T => (val, 1024),
            _ => panic!("Should have failed in check_write_unsupported_register_flags."),
        };
        self.timer_clk = self.timer_count as usize * self.timer_factor as usize;
    }
}

impl InMemory6532 {
    fn one_tick(&mut self) {
        if self.timer_clk != 0 {
            self.timer_clk = self.timer_clk - 1;
            self.timer_count = self.timer_clk.div_ceil(self.timer_factor as usize) as u8;
        }
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn check_read_unsupported_register_flags(&self, addr: usize) {
        let (r, _, name) = regs::IMPLEMENTED_REGISTERS[addr - IOT_MIN_ADDRESS];

        assert!(r, "Reading {name} ({addr:02X}) is not implemented yet.")
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn check_write_unsupported_register_flags(&self, addr: usize, val: u8) {
        let (_, w, name) = regs::IMPLEMENTED_REGISTERS[addr - IOT_MIN_ADDRESS];

        assert!(
            val == 0x00 || w,
            "Writing {name} ({addr:02X}) is not implemented yet, Value 0x{val:02X}."
        )
    }
}

pub mod regs {
    /// Port A; input or output (read or write)
    pub const SWCHA: usize = 0x0280;
    /// Port A DDR, 0= input, 1=output
    pub const SWACNT: usize = 0x0281;
    /// Port B; console switches (read only)
    pub const SWCHB: usize = 0x0282;
    /// Port B DDR (hardwired as input)
    pub const SWBCNT: usize = 0x0283;
    /// Timer output (read only)
    pub const INTIM: usize = 0x0284;
    pub const TIMINT: usize = 0x0285;
    pub const RX0286: usize = 0x0286;
    pub const RX0287: usize = 0x0287;
    pub const RX0288: usize = 0x0288;
    pub const RX0289: usize = 0x0289;
    pub const RX028A: usize = 0x028A;
    pub const RX028B: usize = 0x028B;
    pub const RX028C: usize = 0x028C;
    pub const RX028D: usize = 0x028D;
    pub const RX028E: usize = 0x028E;
    pub const RX028F: usize = 0x028F;
    pub const RX0290: usize = 0x0290;
    pub const RX0291: usize = 0x0291;
    pub const RX0292: usize = 0x0292;
    pub const RX0293: usize = 0x0293;
    /// set 1 clock interval (838 nsec/interval)
    pub const TIM1T: usize = 0x0294;
    /// set 8 clock interval (6.7 usec/interval)
    pub const TIM8T: usize = 0x0295;
    /// set 64 clock interval (53.6 usec/interval)
    pub const TIM64T: usize = 0x0296;
    /// set 1024 clock interval (858.2 usec/interval)
    pub const T1024T: usize = 0x0297;
    pub const RX0298: usize = 0x0298;
    pub const RX0299: usize = 0x0299;
    pub const RX029A: usize = 0x029A;
    pub const RX029B: usize = 0x029B;
    pub const RX029C: usize = 0x029C;
    pub const RX029D: usize = 0x029D;
    pub const RX029E: usize = 0x029E;
    pub const RX029F: usize = 0x029F;

    #[rustfmt::skip]
    pub static IMPLEMENTED_REGISTERS: &[(bool, bool, &str); super::IOT_MAX_ADDRESS - super::IOT_MIN_ADDRESS + 1] = &[
        // R      W     Name
        (false, false, "SWCHA"),   // 0x280	SWCHA	Port A; input or output (read or write)
        (false, false, "SWACNT"),  // 0x281	SWACNT	Port A DDR, 0= input, 1=output
        (false, false, "SWCHB"),   // 0x282	SWCHB	Port B; console switches (read only)
        (false, false, "SWBCNT"),  // 0x283	SWBCNT	Port B DDR (hardwired as input)
        (true , false, "INTIM"),   // 0x284	INTIM	Timer output (read only)
        (false, false, "TIMINT"),  // 0x285 TIMINT  ???
        (false, false, "RX0286"),  // 
        (false, false, "RX0287"),  // 
        (false, false, "RX0288"),  // 
        (false, false, "RX0289"),  // 
        (false, false, "RX028A"),  // 
        (false, false, "RX028B"),  // 
        (false, false, "RX028C"),  // 
        (false, false, "RX028D"),  // 
        (false, false, "RX028E"),  // 
        (false, false, "RX028F"),  // 
        (false, false, "RX0290"),  // 
        (false, false, "RX0291"),  // 
        (false, false, "RX0292"),  // 
        (false, false, "RX0293"),  // 
        (false, true , "TIM1T"),   // 0x294	TIM1T	set 1 clock interval (838 nsec/interval)
        (false, true , "TIM8T"),   // 0x295	TIM8T	set 8 clock interval (6.7 usec/interval)
        (false, true , "TIM64T"),  // 0x296	TIM64T	set 64 clock interval (53.6 usec/interval)
        (false, true , "T1024T"),  // 0x297	T1024T	set 1024 clock interval (858.2 usec/interval)
        (false, false, "RX0298"),  // 
        (false, false, "RX0299"),  // 
        (false, false, "RX029A"),  // 
        (false, false, "RX029B"),  // 
        (false, false, "RX029C"),  // 
        (false, false, "RX029D"),  // 
        (false, false, "RX029E"),  // 
        (false, false, "RX029F"),  // 
    ];
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
