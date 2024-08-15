use a2600::{cpu, mem};
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("D:/bin/Stella-6.7.1/roms/air_raid.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut mem = mem::Memory::new(&buffer, true);
    let pc_lo = mem.get(mem::RESET_VECTOR_LO, mem::RESET_VECTOR_HI);
    let pc_hi = mem.get(mem::RESET_VECTOR_LO + 1, mem::RESET_VECTOR_HI);

    let mut cpu = cpu::MCS6502::default();
    cpu.fetch_decode_execute(&mut mem, pc_lo, pc_hi);
    println!("Hello, world!");

    Ok(())
}
