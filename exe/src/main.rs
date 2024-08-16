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

fn hw_debugger_callback(opc: u8, cpu: &mut cpu::MCS6502, mem: &mut mem::Memory) -> bool {
    dump_registers(opc, cpu);
    loop {
        match repl::get_cmdline().command() {
            Some(repl::Commands::Quit) => {
                return false;
            }
            Some(repl::Commands::Go) => {
                break;
            }
            Some(repl::Commands::Registers) => {
                dump_registers(opc, cpu);
            }
            Some(repl::Commands::DumpMem { start }) => {
                let (lo, hi) = match start {
                    Some(start) => {
                        let start = u128::from_str_radix(start, 16).unwrap_or_default();
                        mem::addr_u16_to_u8(start as u16)
                    }
                    None => (mem::RAM_START_LO, mem::RAM_START_HI),
                };
                dump_memory(mem, lo, hi);
            }
            None => {}
        }
    }

    true
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

fn dump_memory(mem: &mut mem::Memory, start_lo: u8, start_hi: u8) {
    let safe_incr = |s: u8, r: u8, o: u8| ((s as u16) + 16u16 * (r as u16) + (o as u16)) as u8;

    for r in 0..8 {
        let line = (0..16).fold(String::new(), |acc, e| {
            let addr = mem::addr_u16_to_u8(mem::make_addr(safe_incr(start_lo, r, e), start_hi));
            acc + format!(
                "{:02x} {}",
                mem.get(addr.0, addr.1),
                if e == 7 { "- " } else { "" }
            )
            .as_str()
        });
        let addr = mem::addr_u16_to_u8(mem::make_addr(safe_incr(start_lo, r, 0), start_hi));
        println!("[{:02x}:{:02x}] {line}", addr.1, addr.0)
    }
}

fn bit_value(cpu: &cpu::MCS6502, bit: cpu::PSR) -> &str {
    if cpu.p() & bit.bits() == bit.bits() {
        "1"
    } else {
        "0"
    }
}

#[allow(dead_code)]
fn should_break_into_hw_debugger() -> bool {
    let mut buffer = [0u8; 1];
    matches!(io::stdin().read(&mut buffer), Ok(n) if n > 0 && buffer[0] as char == 'p')
}
