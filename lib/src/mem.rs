pub const ADDRESSABLE_MEMORY_SIZE: usize = 0x1_0000;
pub const TOTAL_MEMORY_SIZE: usize = 0x2000;
pub const RESET_VECTOR_LO: u8 = 0xFC;
pub const RESET_VECTOR_HI: u8 = 0xFF;
pub const CARTRIDGE_ROM_START: usize = 0x1000;

/// 2600 Memory map:
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
pub struct Memory {
    data: [u8; TOTAL_MEMORY_SIZE],
}

/// Refer: https://forums.atariage.com/topic/27190-session-5-memory-architecture/
fn make_addr(lo: u8, hi: u8) -> u16 {
    let mut addr = ((hi as u16) << 8) + lo as u16;
    // Step 0. Turn off A13-A15 pins.
    addr &= 0b0001_1111_1111_1111;

    addr
}

// TODO: Implement memory map & checks.
// TODO: Implement mirroring.
// TODO: Implement 2K cartridge
// TODO: Implement 4K+ cartridge
impl Memory {
    pub fn new(cartridge: &[u8]) -> Self {
        if cartridge.len() != 0x1000 {
            unimplemented!("Cartridges other than 4K haven't been implemented yet.")
        }

        let mut data = [0u8; TOTAL_MEMORY_SIZE]; // TODO: Initialize to baadf00d and deadbeef.
        data[CARTRIDGE_ROM_START..CARTRIDGE_ROM_START + cartridge.len()].copy_from_slice(cartridge);
        Self { data }
    }

    pub fn get(&self, lo: u8, hi: u8) -> u8 {
        self.data[make_addr(lo, hi) as usize]
    }

    pub fn set(&mut self, lo: u8, hi: u8, value: u8) {
        self.data[make_addr(lo, hi) as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0x00, 0x00, 0x0000; "no mirroring - 1")]
    #[test_case(0xff, 0x1f, 0x1fff; "no mirroring - 2")]
    #[test_case(0x00, 0x20, 0x0000; "higher half of address space - 1")]
    #[test_case(RESET_VECTOR_LO, RESET_VECTOR_HI, 0x1FFC; "higher half of address space - 2")]
    fn test_make_addr(lo: u8, hi: u8, addr: u16) {
        let ret = make_addr(lo, hi);
        assert_eq!(ret, addr);
    }

    #[test]
    fn test_mem_get_set() {
        let mut mem = Memory::new(&[0b01010101; 0x1000]);
        assert_eq!(mem.get(0x00, 0x11), 0b01010101);
        mem.set(0xA0, 0x00, 0xFE);
        assert_eq!(mem.get(0xA0, 0x00), 0xFE);
    }
}
