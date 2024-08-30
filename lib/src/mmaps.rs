use super::{am, cmn};

/// Base 6502 Memory layout
pub fn mm_6502(a: (u8, u8)) -> usize {
    am::utils::u8_to_u16(a.0, a.1) as usize
}

/// 6507 Memory layout
///
/// 0000-002C TIA (Write)
/// 0030-003D TIA (Read)
/// 0080-00FF RIOT RAM
/// 0280-0297 RIOT I/O, TIMER
/// ... Mirrored (see details below)
/// 1000-1FFF Cartridge ROM
///
/// Details:
/// 0000-003F = TIA Addresses $00-$3F   (zero page)
/// 0040-007F = TIA Addresses $00-$3F   (mirror)
/// 0080-00FF = RIOT RAM                (zero page)
/// 0100-013F = TIA Addresses $00-$3F   (mirror)
/// 0140-017F = TIA Addresses $00-$3F   (mirror)
/// 0180-01FF = RIOT RAM                (mirror)
/// 0200-023F = TIA Addresses $00-$3F   (mirror)
/// 0240-027F = TIA Addresses $00-$3F   (mirror)
/// 0280-029F = RIOT Addresses $00-$1F
/// 02A0-02BF = RIOT Addresses $00-$1F  (mirror)
/// 02C0-02DF = RIOT Addresses $00-$1F  (mirror)
/// 02E0-02FF = RIOT Addresses $00-$1F  (mirror)
/// 0300-033F = TIA Addresses $00-$3F   (mirror)
/// 0340-037F = TIA Addresses $00-$3F   (mirror)
/// 0380-039F = RIOT Addresses $00-$1F  (mirror)
/// 03A0-03BF = RIOT Addresses $00-$1F  (mirror)
/// 03C0-03DF = RIOT Addresses $00-$1F  (mirror)
/// 03E0-03FF = RIOT Addresses $00-$1F  (mirror)
///
/// Pins A12-A15 disabled, so 0300-FFFF mirrored as per https://forums.atariage.com/topic/192418-mirrored-memory/#comment-2439795
///
/// Refer:
/// - https://forums.atariage.com/topic/192418-mirrored-memory/#comment-2439795
/// - https://forums.atariage.com/topic/27190-session-5-memory-architecture/#comment-442653
/// - https://wilsonminesco.com/6502primer/MemMapReqs.html
pub fn mm_6507(a: (u8, u8)) -> usize {
    let mut addr = am::utils::u8_to_u16(a.0, a.1);
    // Step 0. Turn off A13-A15 pins.
    addr &= 0b0001_1111_1111_1111;

    // Step 1. If not in cartridge ROM, turn off
    if addr < am::utils::addr_to_u16(cmn::ROM_START_6507) {
        addr &= 0b0011_1111_1111;
    }

    // Step 2. Implement mirrors in 0000-03FF range
    // TODO: Implement memory map & checks.
    // TODO: Implement mirroring.
    // TODO: Implement 2K cartridges

    addr as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0x00, 0x00, 0x0000; "no mirroring - 1")]
    #[test_case(0xff, 0x1f, 0x1fff; "no mirroring - 2")]
    #[test_case(0x00, 0x20, 0x0000; "higher half of address space - 1")]
    #[test_case(cmn::RESET_VECTOR.0, cmn::RESET_VECTOR.1, 0x1FFC; "higher half of address space - 2")]
    #[test_case(0xfe, 0x07, 0x3fe; "TIA-RAM-RIOT mirror - 1")]
    #[test_case(0x01, 0x08, 0x001; "TIA-RAM-RIOT mirror - 2")]
    #[test_case(0x80, 0x0d, 0x180; "TIA-RAM-RIOT mirror - 3")]
    fn test_resolve_addr(lo: u8, hi: u8, addr: usize) {
        let ret = mm_6507((lo, hi));
        assert_eq!(ret, addr);
    }
}
