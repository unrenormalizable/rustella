use alloc::rc::Rc;
use core::{
    cell::{Cell, RefCell},
    fmt::Debug,
};

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoHi(pub u8, pub u8);

impl core::fmt::Debug for LoHi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:02X}{:02X}", self.1, self.0)
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LineState {
    #[default]
    Low,
    High,
}

pub type Line = Rc<Cell<LineState>>;

pub trait RefExtensions {
    fn rc_cell(self) -> Rc<Cell<Self>>
    where
        Self: Sized,
    {
        Rc::new(Cell::new(self))
    }

    fn rc_refcell(self) -> Rc<RefCell<Self>>
    where
        Self: Sized,
    {
        Rc::new(RefCell::new(self))
    }
}

impl<T> RefExtensions for T {}

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
