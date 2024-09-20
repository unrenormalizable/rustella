use crate::{cmn::*, riot::cmn, riot::mmaps, tia};
use alloc::rc::Rc;
use core::{cell::RefCell, mem};

/// 6502 Memory map: https://wilsonminesco.com/6502primer/MemMapReqs.html
pub struct Memory {
    data: [u8; cmn::TOTAL_MEMORY_SIZE],
    map: fn(LoHi) -> usize,
    tia: Option<Rc<RefCell<dyn cmn::MemorySegment>>>,
}

impl Memory {
    pub fn new(init: bool) -> Self {
        Self::new_with_rom(&[], Default::default(), mmaps::mm_6502, None, init)
    }

    pub fn new_with_rom(
        rom: &[u8],
        rom_start: LoHi,
        map: fn(LoHi) -> usize,
        tia: Option<Rc<RefCell<dyn cmn::MemorySegment>>>,
        init: bool,
    ) -> Self {
        let mut data = [0u8; cmn::TOTAL_MEMORY_SIZE];
        if init {
            Self::fill_with_pattern(&mut data, 0xdeadbeef_baadf00d)
        }

        let mut ret = Self { data, map, tia };
        ret.load(rom, rom_start);

        ret
    }

    #[inline]
    pub fn get(&self, addr: LoHi, index: u8) -> u8 {
        let addr = (self.map)(addr + index);

        if self.tia.is_some() && addr <= tia::TIA_MAX_ADDRESS {
            self.tia.as_ref().unwrap().borrow().read(addr)
        } else {
            self.data[addr]
        }
    }

    #[inline]
    pub fn set(&mut self, addr: LoHi, index: u8, value: u8) {
        let addr = (self.map)(addr + index);

        if self.tia.is_some() && addr <= tia::TIA_MAX_ADDRESS {
            self.tia.as_ref().unwrap().borrow_mut().write(addr, value);
        } else {
            self.data[addr] = value;
        }
    }

    fn fill_with_pattern(data: &mut [u8], pattern: u64) {
        let pattern_bytes = pattern.to_be_bytes();
        let pattern_size = mem::size_of_val(&pattern);
        for word in data.chunks_exact_mut(pattern_size) {
            word[..pattern_size].copy_from_slice(&pattern_bytes[..pattern_size]);
        }
    }

    pub fn load(&mut self, bytes: &[u8], start: LoHi) {
        let start = (self.map)(start);
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
