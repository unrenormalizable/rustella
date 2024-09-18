use crate::cmn::LoHi;

pub const NMI_VECTOR: LoHi = LoHi(0xFA, 0xFF);
pub const RST_VECTOR: LoHi = LoHi(0xFC, 0xFF);
pub const IRQ_VECTOR: LoHi = LoHi(0xFE, 0xFF);

pub struct OpCodeInfo<'a> {
    pub addressing: &'a str,
    pub assembler: &'a str,
    pub bytes: u8,
    pub cycles: usize,
}
