pub fn safe_add(val1: u8, val2: u8) -> (u8, bool) {
    let res = val1 as u16 + val2 as u16;

    let v = res & 0b1_0000_0000 != 0;

    (res as u8, v)
}

pub fn safe_sub(val1: u8, val2: u8) -> (u8, bool) {
    let res = val1 as i16 - val2 as i16;

    let v = res & 0b1_0000_0000 != 0;

    (res as u8, v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0x10, 0x50, (0x60, false))]
    #[test_case(0xfe, 0x01, (0xff, false))]
    #[test_case(0xff, 0x01, (0x00, true))]
    #[test_case(0xfe, 0x11, (0x0f, true))]
    fn test_safe_add(v1: u8, v2: u8, exp: (u8, bool)) {
        let obt = safe_add(v1, v2);
        assert_eq!(exp, obt);
    }

    #[test_case(0x10, 0x10, (0x00, false))]
    #[test_case(0x00, 0x01, (0xFF, true))]
    #[test_case(0x10, 0x20, (0xF0, true))]
    fn test_safe_sub(v1: u8, v2: u8, exp: (u8, bool)) {
        let obt = safe_sub(v1, v2);
        assert_eq!(exp, obt);
    }
}
