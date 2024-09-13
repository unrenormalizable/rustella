use crate::cmn::*;
use crate::mem::mmaps;
use core::mem;

/// 6502 Memory map: https://wilsonminesco.com/6502primer/MemMapReqs.html
pub struct Memory {
    data: [u8; TOTAL_MEMORY_SIZE],
    mmap_fn: MemMapFn,
}

impl Memory {
    pub fn new(init: bool) -> Self {
        Self::new_with_rom(&[], Default::default(), mmaps::mm_6502, init)
    }

    pub fn new_with_rom(rom: &[u8], rom_start: LoHi, mmap_fn: MemMapFn, init: bool) -> Self {
        let mut data = [0u8; TOTAL_MEMORY_SIZE];
        if init {
            Self::fill_with_pattern(&mut data, 0xdeadbeef_baadf00d)
        }

        let mut ret = Self { data, mmap_fn };
        ret.load(rom, rom_start);

        ret
    }

    pub fn get(&self, addr: LoHi, index: u8) -> u8 {
        let addr = addr + index;
        self.data[(self.mmap_fn)(addr)]
    }

    pub fn set(&mut self, addr: LoHi, index: u8, value: u8) {
        let addr = addr + index;
        self.data[(self.mmap_fn)(addr)] = value;
    }

    fn fill_with_pattern(data: &mut [u8], pattern: u64) {
        let pattern_bytes = pattern.to_be_bytes();
        let pattern_size = mem::size_of_val(&pattern);
        for word in data.chunks_exact_mut(pattern_size) {
            word[..pattern_size].copy_from_slice(&pattern_bytes[..pattern_size]);
        }
    }

    pub fn load(&mut self, bytes: &[u8], start: LoHi) {
        let start = u16::from(start) as usize;
        self.data[start..start + bytes.len()].copy_from_slice(bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_wrap_around() {
        let mut mem = Memory::new(true);
        let addr = 0x0100u16;
        assert_eq!(mem.get(addr.into(), 0), 0xDE);
        mem.set(0x00FFu16.into(), 1, 0x99);
        assert_eq!(mem.get(0x00FFu16.into(), 1), 0x99);
        assert_eq!(mem.get(addr.into(), 0), 0x99);
    }

    #[test]
    fn test_address_space_wrap_around() {
        let mut mem = Memory::new(true);
        let addr = 0x0000u16;
        assert_eq!(mem.get(addr.into(), 0), 0xDE);
        mem.set(0xFFFFu16.into(), 1, 0x99);
        assert_eq!(mem.get(0xFFFFu16.into(), 1), 0x99);
        assert_eq!(mem.get(addr.into(), 0), 0x99);
    }

    #[test]
    fn test_mem_get_set() {
        let mut mem = Memory::new(true);
        assert_eq!(mem.get(LoHi(0x00, 0x11), 0), 0xDE);
        mem.set(LoHi(0xA0, 0x00), 0, 0xFE);
        assert_eq!(mem.get(LoHi(0xA0, 0x00), 0), 0xFE);
    }
}
