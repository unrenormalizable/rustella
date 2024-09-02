use super::color_term::VTerm;
use a2600::{cmn::*, cpu, mem, opc_info};
use std::collections::HashSet;

pub fn dump_registers(cpu: &cpu::MOS6502, mem: &mem::Memory, bps: &HashSet<LoHi>) {
    let pc = cpu.pc();
    let (_, bytes_str, instr_str, _, _) = disassemble_one_instruction(mem, bps, pc);

    println!("{}", " PC   AC XR YR SR SP  NV-BDIZC".fg_green());
    println!(
        "{:04X}  {:02X} {:02X} {:02X} {:02X} {:02X}  {}{}{}{}{}{}{}{}",
        u16::from(cpu.pc()),
        cpu.a(),
        cpu.x(),
        cpu.y(),
        cpu.p(),
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

pub fn dump_memory(mem: &mem::Memory, start: &Option<String>) {
    let start = parse_hex_addr_opt(start, RAM_START);

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
    start: &Option<String>,
) {
    let mut pc = parse_hex_addr_opt(start, LoHi(cpu.pc().0, cpu.pc().1));

    let mut instr_len = 0u8;
    for _ in 0..16 {
        pc += instr_len;
        let (opc, bytes_str, instr_str, addr_mode, cycles) =
            disassemble_one_instruction(mem, bps, pc);
        instr_len = opc_info::ALL[opc as usize].bytes;
        println!(
            "{} │ {} │ {} │ {: <7}",
            bytes_str, instr_str, cycles, addr_mode
        )
    }
}

pub fn load(mem: &mut mem::Memory, start: &str, path: &str) {
    let start = u16::from_str_radix(start, 16).map(LoHi::from);
    if start.is_err() {
        println!("Unable to parse start address {:?}", start);
        return;
    }

    let bytes = std::fs::read(path);
    if bytes.is_err() {
        println!("Unable to read file {path}");
        return;
    }

    mem.load(&bytes.unwrap(), start.unwrap());
}

pub fn set_register(cpu: &mut cpu::MOS6502, reg: &str, val: &str) {
    let reg_val = u16::from_str_radix(val, 16);
    if reg_val.is_err() {
        println!("Unable to parse value {val}");
        return;
    }

    let reg_val = reg_val.unwrap();
    match reg.to_lowercase().as_str() {
        "a" => cpu.set_a(reg_val as u8),
        "x" => cpu.set_x(reg_val as u8),
        "y" => cpu.set_y(reg_val as u8),
        "pc" => cpu.set_pc(LoHi::from(reg_val)),
        "s" => cpu.set_s(reg_val as u8),
        "p" => cpu.set_p(reg_val as u8),
        _ => println!("Unknown register {reg}"),
    }
}

pub fn bp_create_or_delete(break_points: &mut HashSet<LoHi>, op: &str, addr: &str) {
    match op.to_lowercase().as_str() {
        "a" | "add" => {
            u16::from_str_radix(addr, 16)
                .ok()
                .map(|x| break_points.insert(LoHi::from(x)));
        }
        "d" | "del" => {
            u16::from_str_radix(addr, 16)
                .ok()
                .map(|x| break_points.remove(&LoHi::from(x)));
        }
        _ => println!("Unknown command {op}"),
    }
}

pub fn bp_list(break_points: &HashSet<LoHi>) {
    break_points
        .iter()
        .enumerate()
        .for_each(|(n, bp)| println!("{n:02} {:02X}{:02X}", bp.1, bp.0))
}

fn disassemble_one_instruction(
    mem: &mem::Memory,
    bps: &HashSet<LoHi>,
    pc: LoHi,
) -> (u8, String, String, &'static str, u64) {
    let opc = mem.get(pc, 0);
    let opc_info = &opc_info::ALL[opc as usize];
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
    if cpu::tst_bit(cpu.p(), bit.bits()) {
        "1".to_string()
    } else {
        "0".to_lowercase()
    }
}

fn parse_hex_addr_opt(val: &Option<String>, default: LoHi) -> LoHi {
    val.as_ref()
        .and_then(|x| u16::from_str_radix(x, 16).ok())
        .map(LoHi::from)
        .unwrap_or(default)
}

fn clock_speed(cpu: &cpu::MOS6502) -> f64 {
    if cpu.duration() != 0 {
        (cpu.cycles() as f64 * 1_000_000_000.0) / cpu.duration() as f64 / 1_000_000.0
    } else {
        0.0
    }
}
