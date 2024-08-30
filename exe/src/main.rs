mod cmds;
mod repl;

use a2600::{cmn, cpu, mem, mmaps};
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), String> {
    let buffer = read_cartridge_rom();
    let mut mem = mem::Memory::new_with_rom(&buffer, cmn::ROM_START_6507, mmaps::mm_6507, true);
    let mut cpu = cpu::MOS6502::default();
    cpu.fetch_decode_execute(&mut mem, hw_debugger_callback);

    Ok(())
}

fn read_cartridge_rom() -> Vec<u8> {
    let mut file = File::open("D:/bin/Stella-6.7.1/roms/air_raid.bin").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    buffer
}

fn hw_debugger_callback(_: u8, cpu: &mut cpu::MOS6502, mem: &mut mem::Memory) -> (bool, usize) {
    cmds::dump_registers(cpu, mem);
    loop {
        match repl::get_cmdline().command() {
            Some(repl::Commands::Quit) => {
                return (false, 0);
            }
            Some(repl::Commands::Go { count }) => {
                return (true, count.unwrap_or(1));
            }
            Some(repl::Commands::Registers) => {
                cmds::dump_registers(cpu, mem);
            }
            Some(repl::Commands::MemoryDump { start }) => {
                cmds::dump_memory(mem, start);
            }
            Some(repl::Commands::Disassemble { start }) => {
                cmds::disassemble(cpu, mem, start);
            }
            Some(repl::Commands::Load { start, path: file }) => {
                cmds::load(cpu, mem, start, file);
            }
            Some(repl::Commands::SetReg { reg, val }) => {
                cmds::set_register(cpu, mem, reg, val);
            }
            None => {}
        }
    }
}
