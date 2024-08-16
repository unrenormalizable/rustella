mod repl;

use a2600::{cpu, hw_dbg, mem};
use std::fs::File;
use std::io::{self, Read};

// TODO:
// - Debugging - 1: RO to go through the flow.
// - Compare with stell up to a 100 instructions.
// - Memory mirroring
// - Debugging - 2: RW

fn main() -> Result<(), String> {
    let buffer = read_cartridge_rom();
    let mut mem = mem::Memory::new(&buffer, true);
    let (pc_lo, pc_hi) = mem.get_pc_from_reset_vector();
    let mut cpu = cpu::MCS6502::new(pc_lo, pc_hi);
    cpu.fetch_decode_execute(&mut mem, hw_debugger_callback);

    Ok(())
}

fn read_cartridge_rom() -> Vec<u8> {
    let mut file = File::open("D:/bin/Stella-6.7.1/roms/air_raid.bin").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    buffer
}

fn dump_registers(opc: u8, cpu: &mut cpu::MCS6502) {
    println!(" PC  | OP                 |  A  X  Y  S | [N V B D I Z C]");
    println!(
        "{:02x}{:02x} | {:02x} {: >15} | {:02x} {:02x} {:02x} {:02x} | [{} {} {} {} {} {} {}]",
        cpu.pc().1,
        cpu.pc().0,
        opc,
        hw_dbg::ALL_OPCODE_INFO[opc as usize].0,
        cpu.a(),
        cpu.x(),
        cpu.y(),
        cpu.s(),
        bit_value(cpu, cpu::PSR::N),
        bit_value(cpu, cpu::PSR::V),
        bit_value(cpu, cpu::PSR::B),
        bit_value(cpu, cpu::PSR::D),
        bit_value(cpu, cpu::PSR::I),
        bit_value(cpu, cpu::PSR::Z),
        bit_value(cpu, cpu::PSR::C),
    );
}

fn bit_value(cpu: &cpu::MCS6502, bit: cpu::PSR) -> &str {
    if cpu.p() & bit.bits() == bit.bits() {
        "1"
    } else {
        "0"
    }
}

fn hw_debugger_callback(opc: u8, cpu: &mut cpu::MCS6502, _mem: &mut mem::Memory) -> bool {
    dump_registers(opc, cpu);
    loop {
        match repl::get_cmdline().command() {
            Some(repl::Commands::Quit) => {
                return false;
            }
            Some(repl::Commands::Go) => {
                break;
            }
            Some(repl::Commands::DumpRam) => {}
            Some(repl::Commands::Registers) => {
                dump_registers(opc, cpu);
            }
            Some(repl::Commands::DumpMem { start: _ }) => {
                println!("Not implemented yet...")
            }
            None => {}
        }
    }

    true
}

#[allow(dead_code)]
fn should_break_into_hw_debugger() -> bool {
    let mut buffer = [0u8; 1];
    matches!(io::stdin().read(&mut buffer), Ok(n) if n > 0 && buffer[0] as char == 'p')
}
