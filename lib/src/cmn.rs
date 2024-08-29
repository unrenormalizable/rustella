pub const TOTAL_MEMORY_SIZE: usize = 0x2000;
pub const ADDRESSABLE_MEMORY_SIZE: usize = 0x1_0000;
pub const RESET_VECTOR_LO: u8 = 0xFC;
pub const RESET_VECTOR_HI: u8 = 0xFF;
pub const RAM_START_LO: u8 = 0x80;
pub const RAM_START_HI: u8 = 0x00;
pub const RAM_SIZE: usize = 0x0080;

pub const ROM_START_6507: usize = 0x1000;

pub struct Address(pub u8, pub u8);
pub type Address2 = (u8, u8);

pub type MemMapFn = fn((u8, u8)) -> usize;

pub struct OpCodeInfo<'a> {
    pub addressing: &'a str,
    pub assembler: &'a str,
    pub bytes: u8,
    pub cycles: &'a str,
}
