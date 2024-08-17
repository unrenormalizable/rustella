mod cmds;
mod repl;

use a2600::{cpu, mem};
use std::fs::File;
use std::io::Read;

// TODO:
// v Debugging - 1: RO to go through the flow.
// - Compare with stell up to a 1000 instructions.
// - Stats: instructions executed, cycles, time
// - Memory mirroring
// - Debugging - 2: RW

fn main() -> Result<(), String> {
    let buffer = read_cartridge_rom();
    let mut mem = mem::Memory::new(&buffer, true);
    let (pc_lo, pc_hi) = mem.get_pc_from_reset_vector();
    let mut cpu = cpu::MOS6502::new(pc_lo, pc_hi);
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
            None => {}
        }
    }
}
