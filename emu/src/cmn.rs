/// https://www.pagetable.com/?p=410
pub const NMI_VECTOR: LoHi = LoHi(0xFA, 0xFF);
/// https://www.pagetable.com/?p=410
pub const RST_VECTOR: LoHi = LoHi(0xFC, 0xFF);
/// https://www.pagetable.com/?p=410
pub const IRQ_VECTOR: LoHi = LoHi(0xFE, 0xFF);

pub const TOTAL_MEMORY_SIZE: usize = 0x1_0000;
pub const ADDRESSABLE_MEMORY_SIZE: usize = 0x1_0000;
pub const RAM_START: LoHi = LoHi(0x80, 0x00);
pub const RAM_SIZE: usize = 0x0080;

pub const ROM_START_6507: LoHi = LoHi(0x00, 0x10);

pub struct OpCodeInfo<'a> {
    pub addressing: &'a str,
    pub assembler: &'a str,
    pub bytes: u8,
    pub cycles: u64,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoHi(pub u8, pub u8);

impl core::fmt::Debug for LoHi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "({:02X}, {:02X})", self.1, self.0)
    }
}

impl core::ops::Add<u8> for LoHi {
    type Output = LoHi;

    #[inline]
    fn add(self, rhs: u8) -> Self::Output {
        u16::from(self).wrapping_add(rhs as u16).into()
    }
}

impl core::ops::AddAssign<u8> for LoHi {
    #[inline]
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

impl From<(u8, u8)> for LoHi {
    #[inline]
    fn from(value: (u8, u8)) -> Self {
        LoHi(value.0, value.1)
    }
}

impl From<u16> for LoHi {
    #[inline]
    fn from(value: u16) -> Self {
        LoHi(value as u8, (value >> 8) as u8)
    }
}

impl From<LoHi> for u16 {
    #[inline]
    fn from(value: LoHi) -> Self {
        ((value.1 as u16) << 8) + value.0 as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0x00, 0x01, LoHi(0x00, 0x01))]
    fn lohi_from_to_u8xu8(lo: u8, hi: u8, exp: LoHi) {
        let val = LoHi::from((lo, hi));
        assert_eq!(val, exp);
    }

    #[test_case(0x0001, LoHi(0x01, 0x00))]
    fn lohi_from_to_u16(lohi: u16, exp: LoHi) {
        let val = LoHi::from(lohi);
        assert_eq!(val, exp);
        assert_eq!(u16::from(val), lohi);
    }

    #[test_case(LoHi(0x01, 0x00), 1, LoHi(0x02, 0x00))]
    #[test_case(LoHi(0xff, 0x00), 1, LoHi(0x00, 0x01))]
    #[test_case(LoHi(0xf0, 0x00), 0x10, LoHi(0x00, 0x01))]
    #[test_case(LoHi(0xff, 0xff), 1, LoHi(0x00, 0x00))]
    fn lohi_add(lohi: LoHi, delta: u8, exp: LoHi) {
        let val = lohi + delta;
        assert_eq!(val, exp);
    }
}
