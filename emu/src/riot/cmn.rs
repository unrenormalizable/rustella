pub const TOTAL_MEMORY_SIZE: usize = 0x1_0000;

pub trait MemorySegment {
    fn read(&self, addr: usize) -> u8;
    fn write(&mut self, addr: usize, val: u8);
}

pub const IOT_MIN_ADDRESS: usize = 0x0280;
pub const IOT_MAX_ADDRESS: usize = 0x029F;

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
    pub static IMPLEMENTED_REGISTERS: &[(bool, bool, &str, u8); super::IOT_MAX_ADDRESS - super::IOT_MIN_ADDRESS + 1] = &[
        // R      W     Name     Supported Mask
        (false, false, "SWCHA",  0b_0000_0000),  // 0x280	SWCHA	Port A; input or output (read or write)
        (false, false, "SWACNT", 0b_0000_0000),  // 0x281	SWACNT	Port A DDR, 0= input, 1=output
        (true,  false, "SWCHB",  0b_0000_0000),  // 0x282	SWCHB	Port B; console switches (read only)
        (false, false, "SWBCNT", 0b_0000_0000),  // 0x283	SWBCNT	Port B DDR (hardwired as input)
        (true , false, "INTIM",  0b_0000_0000),  // 0x284	INTIM	Timer output (read only)
        (false, false, "TIMINT", 0b_0000_0000),  // 0x285   TIMINT  ???
        (false, false, "RX0286", 0b_0000_0000),  // 
        (false, false, "RX0287", 0b_0000_0000),  // 
        (false, false, "RX0288", 0b_0000_0000),  // 
        (false, false, "RX0289", 0b_0000_0000),  // 
        (false, false, "RX028A", 0b_0000_0000),  // 
        (false, false, "RX028B", 0b_0000_0000),  // 
        (false, false, "RX028C", 0b_0000_0000),  // 
        (false, false, "RX028D", 0b_0000_0000),  // 
        (false, false, "RX028E", 0b_0000_0000),  // 
        (false, false, "RX028F", 0b_0000_0000),  // 
        (false, false, "RX0290", 0b_0000_0000),  // 
        (false, false, "RX0291", 0b_0000_0000),  // 
        (false, false, "RX0292", 0b_0000_0000),  // 
        (false, false, "RX0293", 0b_0000_0000),  // 
        (false, true , "TIM1T",  0b_1111_1111),  // 0x294	TIM1T	set 1 clock interval (838 nsec/interval)
        (false, true , "TIM8T",  0b_1111_1111),  // 0x295	TIM8T	set 8 clock interval (6.7 usec/interval)
        (false, true , "TIM64T", 0b_1111_1111),  // 0x296	TIM64T	set 64 clock interval (53.6 usec/interval)
        (false, true , "T1024T", 0b_1111_1111),  // 0x297	T1024T	set 1024 clock interval (858.2 usec/interval)
        (false, false, "RX0298", 0b_0000_0000),  // 
        (false, false, "RX0299", 0b_0000_0000),  // 
        (false, false, "RX029A", 0b_0000_0000),  // 
        (false, false, "RX029B", 0b_0000_0000),  // 
        (false, false, "RX029C", 0b_0000_0000),  // 
        (false, false, "RX029D", 0b_0000_0000),  // 
        (false, false, "RX029E", 0b_0000_0000),  // 
        (false, false, "RX029F", 0b_0000_0000),  // 
    ];
}
