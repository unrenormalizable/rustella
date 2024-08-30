#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoHi(pub u8, pub u8);

pub const TOTAL_MEMORY_SIZE: usize = 0x1_0000;
pub const ADDRESSABLE_MEMORY_SIZE: usize = 0x1_0000;
pub const RESET_VECTOR: LoHi = LoHi(0xFC, 0xFF);
pub const RAM_START: LoHi = LoHi(0x80, 0x00);
pub const RAM_SIZE: usize = 0x0080;

pub const ROM_START_6507: LoHi = LoHi(0x00, 0x10);

pub type MemMapFn = fn((u8, u8)) -> usize;

pub struct OpCodeInfo<'a> {
    pub addressing: &'a str,
    pub assembler: &'a str,
    pub bytes: u8,
    pub cycles: &'a str,
}
