use crate::cmn::*;

/// Base 6502 Memory layout
pub fn mm_6502(a: LoHi) -> usize {
    u16::from(a) as usize
}

/// 6507 Memory layout:
/// - https://www.qotile.net/minidig/docs/2600_mem_map.txt
/// - https://www.taswegian.com/WoodgrainWizard/tiki-index.php?page=Memory-Map
///
/// Pins A12-A15 disabled, so 0300-FFFF mirrored as per https://forums.atariage.com/topic/192418-mirrored-memory/#comment-2439795
///
/// Refer:
/// - https://forums.atariage.com/topic/192418-mirrored-memory/#comment-2439795
/// - https://forums.atariage.com/topic/27190-session-5-memory-architecture/#comment-442653
/// - https://wilsonminesco.com/6502primer/MemMapReqs.html
pub fn mm_6507(a: LoHi) -> usize {
    let mut addr = u16::from(a) as usize;
    // Step 0. Turn off A13-A15 pins.
    addr &= 0b0001_1111_1111_1111;

    // Step 1. If not in cartridge ROM, turn off
    if addr < u16::from(ROM_START_6507) as usize {
        addr &= 0b0011_1111_1111;
    }

    // Step 2. Implement mirrors in 0000-03FF range
    // TODO: Implement memory map & checks.
    // TODO: Implement mirroring.
    // TODO: Implement 2K cartridges

    addr
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(LoHi(0x00, 0x00), 0x0000; "no mirroring - 1")]
    #[test_case(LoHi(0xff, 0x1f), 0x1fff; "no mirroring - 2")]
    #[test_case(LoHi(0x00, 0x20), 0x0000; "higher half of address space - 1")]
    #[test_case(RST_VECTOR, 0x1FFC; "higher half of address space - 2")]
    #[test_case(LoHi(0xfe, 0x07), 0x3fe; "TIA-RAM-RIOT mirror - 1")]
    #[test_case(LoHi(0x01, 0x08), 0x001; "TIA-RAM-RIOT mirror - 2")]
    #[test_case(LoHi(0x80, 0x0d), 0x180; "TIA-RAM-RIOT mirror - 3")]
    fn test_resolve_addr(lohi: LoHi, addr: usize) {
        let ret = mm_6507(lohi);
        assert_eq!(ret, addr);
    }
}
