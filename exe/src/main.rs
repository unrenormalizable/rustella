mod cmds;
mod repl;

use a2600::{cmn, cpu, mem, mmaps};
use std::{collections::HashSet, fs};

// TODO: Abstract out hd_dbg module.
fn main() -> Result<(), String> {
    let buffer = fs::read("d:/src/u/a2600/AllSuiteA.bin").unwrap();
    let mut mem = mem::Memory::new_with_rom(&buffer, cmn::LoHi(0x00, 0x40), mmaps::mm_6502, true);
    //let buffer = fs::read("D:/bin/Stella-6.7.1/roms/air_raid.bin").unwrap();
    //let mut mem = mem::Memory::new_with_rom(&buffer, cmn::ROM_START_6507, mmaps::mm_6507, true);
    let mut cpu = cpu::MOS6502::new(&mem);

    let mut break_points = HashSet::new();

    cmds::dump_registers(&cpu, &mem, &break_points);
    loop {
        match repl::get_cmdline().command() {
            Some(repl::Commands::Quit) => break,
            Some(repl::Commands::Go { count }) => {
                let mut count = count.unwrap_or(usize::MAX);
                loop {
                    cpu.fetch_decode_execute(&mut mem);
                    count -= 1;

                    if count == 0 {
                        break;
                    }

                    if break_points.contains(&cpu.pc()) {
                        break;
                    }
                }

                cmds::dump_registers(&cpu, &mem, &break_points);
            }
            Some(repl::Commands::Registers) => cmds::dump_registers(&cpu, &mem, &break_points),
            Some(repl::Commands::MemoryDump { start }) => cmds::dump_memory(&mem, start),
            Some(repl::Commands::Disassemble { start }) => {
                cmds::disassemble(&cpu, &mem, &break_points, start)
            }
            Some(repl::Commands::Load { start, path: file }) => cmds::load(&mut mem, start, file),
            Some(repl::Commands::SetReg { reg, val }) => cmds::set_register(&mut cpu, reg, val),
            Some(repl::Commands::BreakPoint { op, addr }) => {
                cmds::bp_create_or_delete(&mut break_points, op, addr)
            }
            Some(repl::Commands::BreakPointList) => cmds::bp_list(&break_points),
            None => {}
        }
    }

    Ok(())
}
