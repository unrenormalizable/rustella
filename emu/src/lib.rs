#![no_std]
extern crate alloc;

pub mod cmn;
pub mod cpu;
pub mod mem;
pub mod tia;

pub mod bits {
    pub const BIT_D0: u8 = 0x01 << 0;
    pub const BIT_D1: u8 = 0x01 << 1;
    pub const BIT_D2: u8 = 0x01 << 2;
    pub const BIT_D3: u8 = 0x01 << 3;
    pub const BIT_D4: u8 = 0x01 << 4;
    pub const BIT_D5: u8 = 0x01 << 5;
    pub const BIT_D6: u8 = 0x01 << 6;
    pub const BIT_D7: u8 = 0x01 << 7;

    #[inline]
    pub fn tst_bits<
        T: num_traits::sign::Unsigned + core::ops::BitAnd + PartialEq + core::marker::Copy,
    >(
        val: T,
        bits_to_test: T,
    ) -> bool
    where
        <T as core::ops::BitAnd>::Output: PartialEq<T>,
    {
        val & bits_to_test == bits_to_test
    }
}

#[cfg(test)]
mod tests {
    macro_rules! check_2D_array {
        ($buffer:expr, $expected:expr) => {
            assert!($buffer.len() > 0 && $buffer[0].len() > 0, "Buffer is empty");
            $buffer.iter().enumerate().for_each(|(r, buf)| {
                buf.iter().enumerate().for_each(|(c, &val)| {
                    let expected = $expected;
                    assert_eq!(
                        val, expected,
                        "[{r},{c}]: Found 0x{val:02X}, expected 0x{expected:02X}."
                    )
                })
            });
        };
        ($buffer:expr, $bounds:expr, $val:expr) => {
            let mut atleast_one_match = false;
            $buffer.iter().enumerate().for_each(|(r, buf)| {
                buf.iter().enumerate().for_each(|(c, &val)| {
                    let expected = if ($bounds.0..$bounds.2).contains(&r)
                        && ($bounds.1..$bounds.3).contains(&c)
                    {
                        atleast_one_match = true;
                        $val.0
                    } else {
                        $val.1
                    };

                    assert_eq!(
                        val, expected,
                        "[{r},{c}]: Found 0x{val:02X}, expected 0x{expected:02X}."
                    )
                })
            });
            assert!(atleast_one_match, "There were no matches. Test is faulty!")
        };
    }

    pub(crate) use check_2D_array;
}
