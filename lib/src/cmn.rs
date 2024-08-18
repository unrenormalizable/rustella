pub fn safe_add_checked(val1: u8, val2: u8) -> (u8, bool) {
    let res = val1 as u16 + val2 as u16;

    let v = res & 0b1_0000_0000 != 0;

    (res as u8, v)
}

pub fn safe_add(val1: u8, val2: u8) -> u8 {
    let res = val1 as u16 + val2 as u16;

    res as u8
}

pub fn safe_add2(val1: u8, val2: u8, val3: u8) -> u8 {
    safe_add(safe_add(val1, val2), val3)
}

pub fn safe_sub_checked(val1: u8, val2: u8) -> (u8, bool) {
    let res = val1 as i16 - val2 as i16;

    let v = res & 0b1_0000_0000 != 0;

    (res as u8, v)
}

pub fn safe_sub(val1: u8, val2: u8) -> u8 {
    let res = val1 as i16 - val2 as i16;

    res as u8
}

pub fn offset_addr(lo: u8, hi: u8, off: u8) -> u16 {
    let addr = (addr_u8_to_u16(lo, hi) as u32) + off as u32;

    addr as u16
}

pub fn addr_u8_to_u16(lo: u8, hi: u8) -> u16 {
    ((hi as u16) << 8) + lo as u16
}

pub fn addr_u16_to_u8(addr: u16) -> (u8, u8) {
    (addr as u8, (addr >> 8) as u8)
}

pub fn indexed(lo: u8, hi: u8, index: u8) -> (u8, u8) {
    if index == 0 {
        (lo, hi)
    } else {
        addr_u16_to_u8(offset_addr(lo, hi, index))
    }
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
        let obt = safe_add_checked(v1, v2);
        assert_eq!(exp, obt);
    }

    #[test_case(0x10, 0x10, (0x00, false))]
    #[test_case(0x00, 0x01, (0xFF, true))]
    #[test_case(0x10, 0x20, (0xF0, true))]
    fn test_safe_sub(v1: u8, v2: u8, exp: (u8, bool)) {
        let obt = safe_sub_checked(v1, v2);
        assert_eq!(exp, obt);
    }

    #[test_case(0x00, 0x00, 0x0000, 0x0000)]
    #[test_case(0xf0, 0x00, 0x0010, 0x0100)]
    #[test_case(0xff, 0xff, 0x0002, 0x0001)]
    fn test_offset_addr(lo: u8, hi: u8, off: u8, exp: u16) {
        let ret = offset_addr(lo, hi, off);
        assert_eq!(ret, exp);
    }

    #[test_case(0x00, 0x00, 0x0000)]
    #[test_case(0xf0, 0x00, 0x00f0)]
    #[test_case(0xff, 0xff, 0xffff)]
    fn test_addr_formats(lo: u8, hi: u8, exp: u16) {
        let ret = addr_u8_to_u16(lo, hi);
        assert_eq!(ret, exp);
        let addru8 = addr_u16_to_u8(ret);
        assert_eq!(addru8, (lo, hi));
    }
}
