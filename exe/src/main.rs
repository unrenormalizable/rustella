mod cmds;
mod color_term;
mod repl;

use a2600::{cmn, cpu, mem, mmaps};
use std::{collections::HashSet, fs};

// TODO: Abstract out hd_dbg module.
fn main() {
    let buffer = fs::read("d:/src/u/a2600/AllSuiteA.bin").unwrap();
    let mut mem = mem::Memory::new_with_rom(&buffer, cmn::LoHi(0x00, 0x40), mmaps::mm_6502, false);
    //let buffer = fs::read("D:/bin/Stella-6.7.1/roms/air_raid.bin").unwrap();
    //let mut mem = mem::Memory::new_with_rom(&buffer, cmn::ROM_START_6507, mmaps::mm_6507, true);
    let mut cpu = cpu::MOS6502::new(&mem);

    let mut break_points = HashSet::new();

    cmds::registers(&cpu, &mem, &break_points);
    let cl = repl::cmd_line();
    cl.repl(|command| match command {
        repl::Commands::Quit => println!("Press Ctrl+C to exit."),
        repl::Commands::Go { count } => cmds::go(&mut cpu, &mut mem, &break_points, count),
        repl::Commands::Registers => cmds::registers(&cpu, &mem, &break_points),
        repl::Commands::SetRegisters { reg, val } => cmds::set_register(&mut cpu, reg, val),
        repl::Commands::Memory { start } => cmds::memory(&mem, start),
        repl::Commands::Disassemble { start } => {
            cmds::disassemble(&cpu, &mem, &break_points, start)
        }
        repl::Commands::Load { start, path } => cmds::load(&mut mem, start, path),
        repl::Commands::BreakPoints => cmds::break_points(&break_points),
        repl::Commands::BreakPointChange { op, address } => {
            cmds::change_break_points(&mut break_points, op, address)
        }
    });
}
