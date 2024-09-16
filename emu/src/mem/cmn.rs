pub const TOTAL_MEMORY_SIZE: usize = 0x1_0000;

pub trait MemorySegment {
    fn read(&self, addr: usize) -> u8;
    fn write(&mut self, addr: usize, val: u8);
}
