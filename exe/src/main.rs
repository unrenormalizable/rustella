use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("D:/bin/Stella-6.7.1/roms/air_raid.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mem = a2600::mem::Memory::new(&buffer);
    a2600::decode(&mem, 0, 0);
    println!("Hello, world!");

    Ok(())
}
