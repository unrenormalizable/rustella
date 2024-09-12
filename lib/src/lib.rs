#![no_std]

pub mod cmn;
pub mod cpu;
pub mod mem;
pub mod tia;

pub mod bits {
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

    macro_rules! check_display {
        ($buffer:expr, $expected:expr) => {
            assert!($buffer.len() > 0 && $buffer[0].len() > 0, "Buffer is empty");
            $buffer.iter().enumerate().for_each(|(r, buf)| {
                buf.iter().enumerate().for_each(|(c, &val)| {
                    let expected = $expected;
                    assert_eq!(val, expected, "[{r},{c}]: Found {val}, expected {expected}")
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

                    assert_eq!(val, expected, "[{r},{c}]: Found {val}, expected {expected}")
                })
            });
            assert!(atleast_one_match, "There were no matches. Test is faulty!")
        };
    }

    pub(crate) use check_display;
}
