use super::{color_term::VTerm, repl};
use rustella::{bits, cmn::LoHi, cpu, mem};
use std::collections::HashSet;
use std::path::PathBuf;

pub fn go(cpu: &mut cpu::MOS6502, mem: &mut mem::Memory, break_points: &HashSet<LoHi>, count: u64) {
    let mut count = count;
    loop {
        cpu.tick(mem);
        count -= 1;

        if count == 0 {
            break;
        }

        if break_points.contains(&cpu.pc()) {
            break;
        }
    }

    registers(cpu, mem, break_points);
}

pub fn registers(cpu: &cpu::MOS6502, mem: &mem::Memory, bps: &HashSet<LoHi>) {
    let pc = cpu.pc();
    let (_, bytes_str, instr_str, _, _) = disassemble_one_instruction(mem, bps, pc);

    println!("{}", " PC   AC XR YR SR SP  NV-BDIZC".fg_green());
    println!(
        "{:04X}  {:02X} {:02X} {:02X} {:02X} {:02X}  {}{}{}{}{}{}{}{}",
        u16::from(cpu.pc()),
        cpu.a(),
        cpu.x(),
        cpu.y(),
        cpu.psr(),
        cpu.s(),
        bit_value(cpu, cpu::PSR::N),
        bit_value(cpu, cpu::PSR::V),
        bit_value(cpu, cpu::PSR::__),
        bit_value(cpu, cpu::PSR::B),
        bit_value(cpu, cpu::PSR::D),
        bit_value(cpu, cpu::PSR::I),
        bit_value(cpu, cpu::PSR::Z),
        bit_value(cpu, cpu::PSR::C),
    );
    println!("{}  {}", bytes_str, instr_str,);
    println!(
        "{} ops, {} cycles @ {:.02} MHz",
        cpu.instructions(),
        cpu.cycles(),
        clock_speed(cpu)
    );
}

pub fn set_register(cpu: &mut cpu::MOS6502, reg: repl::Register, val: u16) {
    match reg {
        repl::Register::A => cpu.set_a(val as u8),
        repl::Register::X => cpu.set_x(val as u8),
        repl::Register::Y => cpu.set_y(val as u8),
        repl::Register::PC => cpu.set_pc(LoHi::from(val)),
        repl::Register::S => cpu.set_s(val as u8),
        repl::Register::PSR => cpu.set_psr(val as u8),
    }
}

pub fn memory(mem: &mem::Memory, start: u16) {
    let start = LoHi::from(start);

    for r in 0..8u8 {
        let line = (0..16u8).fold(String::new(), |acc, e| {
            let addr = start + r.wrapping_mul(16u8) + e;
            acc + format!(
                "{:02X} {}",
                mem.get(addr, 0),
                if e == 7 { "- " } else { "" }
            )
            .as_str()
        });
        let addr = start + r.wrapping_mul(16u8);
        println!("{:04X}: {line}", u16::from(addr))
    }
}

pub fn disassemble(
    cpu: &cpu::MOS6502,
    mem: &mem::Memory,
    bps: &HashSet<LoHi>,
    start: u16,
    count: u64,
) {
    let mut pc = if start == 0 {
        cpu.pc()
    } else {
        LoHi::from(start)
    };

    let mut instr_len = 0u8;
    for _ in 0..count {
        pc += instr_len;
        let (opc, bytes_str, instr_str, addr_mode, cycles) =
            disassemble_one_instruction(mem, bps, pc);
        instr_len = cpu::opc_info::ALL[opc as usize].bytes;
        println!(
            "{} │ {} │ {} │ {: <7}",
            bytes_str, instr_str, cycles, addr_mode
        )
    }
}

pub fn load(mem: &mut mem::Memory, start: u16, path: PathBuf) {
    let start = LoHi::from(start);
    let bytes = std::fs::read(path.clone());
    if bytes.is_err() {
        println!("Unable to read file {:?}", path);
        return;
    }

    mem.load(&bytes.unwrap(), start);
}

pub fn change_break_points(break_points: &mut HashSet<LoHi>, op: repl::BreakPointOp, addr: u16) {
    let addr = LoHi::from(addr);
    match op {
        repl::BreakPointOp::Add => break_points.insert(addr),
        repl::BreakPointOp::Remove => break_points.remove(&addr),
    };
}

pub fn break_points(break_points: &HashSet<LoHi>) {
    break_points
        .iter()
        .enumerate()
        .for_each(|(n, bp)| println!("{n:02} {:02X}{:02X}", bp.1, bp.0))
}

fn disassemble_one_instruction(
    mem: &mem::Memory,
    bps: &HashSet<LoHi>,
    pc: LoHi,
) -> (u8, String, String, &'static str, usize) {
    let opc = mem.get(pc, 0);
    let opc_info = &cpu::opc_info::ALL[opc as usize];
    let instr_b1_str = if opc_info.bytes > 1 {
        &format!("{:02X}", mem.get(pc, 1))
    } else {
        ""
    };
    let instr_b2_str = if opc_info.bytes > 2 {
        &format!("{:02X}", mem.get(pc, 2))
    } else {
        ""
    };

    let pc_str = format!("{:02X}{:02X}", pc.1, pc.0);
    let pc_str = if bps.contains(&pc) {
        pc_str.bg_red()
    } else {
        pc_str
    };
    let bytes_str = format!(
        "{} {:02X} {: <2} {: <2}",
        pc_str, opc, instr_b1_str, instr_b2_str
    );

    let instr_str = format!(
        "{: >10}",
        opc_info
            .assembler
            .replace("oper", (instr_b2_str.to_string() + instr_b1_str).as_str())
    );

    (
        opc,
        bytes_str,
        instr_str,
        opc_info.addressing,
        opc_info.cycles,
    )
}

fn bit_value(cpu: &cpu::MOS6502, bit: cpu::PSR) -> String {
    if bits::tst_bits(cpu.psr(), bit.bits()) {
        "1".to_string()
    } else {
        "0".to_lowercase()
    }
}

fn clock_speed(cpu: &cpu::MOS6502) -> f64 {
    if cpu.duration() != 0 {
        (cpu.cycles() as f64 * 1_000_000_000.0) / cpu.duration() as f64 / 1_000_000.0
    } else {
        0.0
    }
}
