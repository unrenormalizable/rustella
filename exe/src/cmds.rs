use a2600::{cmn, cpu, hw_dbg, mem};

pub fn dump_registers(cpu: &cpu::MCS6502, mem: &mem::Memory) {
    let (pc_lo, pc_hi) = cpu.pc();
    let (_, bytes_str, instr_str, instr_len, addr_mode) =
        disassemble_one_instruction(pc_lo, pc_hi, mem);

    println!("┌────────────────┬────────────┬─────────────┬─────────────────┬──────────────┐");
    println!("│ PC             │ OP         │ A  X  Y  S  │       PSR       │              │");
    println!("╞════════════════╪════════════╪═════════════╪═════════════════╪════╤═════════╡");
    println!(
        "│ {} │ {: <10} │ {:02x} {:02x} {:02x} {:02x} │ {} {} - {} {} {} {} {} │ {: >2x} │ {: >7} │",
        bytes_str,
        instr_str,
        cpu.a(),
        cpu.x(),
        cpu.y(),
        cpu.s(),
        bit_value(cpu, cpu::PSR::N, "N"),
        bit_value(cpu, cpu::PSR::V, "V"),
        bit_value(cpu, cpu::PSR::B, "B"),
        bit_value(cpu, cpu::PSR::D, "D"),
        bit_value(cpu, cpu::PSR::I, "I"),
        bit_value(cpu, cpu::PSR::Z, "Z"),
        bit_value(cpu, cpu::PSR::C, "C"),
        instr_len,
        addr_mode,
    );
    println!("└────────────────┴────────────┴─────────────┴─────────────────┴────┴─────────┘");
}

pub fn dump_memory(mem: &mem::Memory, start: &Option<String>) {
    let (start_lo, start_hi) = match start {
        Some(start) => {
            let start = u128::from_str_radix(start, 16).unwrap_or_default();
            cmn::addr_u16_to_u8(start as u16)
        }
        None => (mem::RAM_START_LO, mem::RAM_START_HI),
    };

    let safe_incr = |s: u8, r: u8, o: u8| {
        // TODO: Potential crash. Change it to use mem:: functions
        ((s as u16) + 16u16 * (r as u16) + (o as u16)) as u8
    };

    for r in 0..8 {
        let line = (0..16).fold(String::new(), |acc, e| {
            let addr = (safe_incr(start_lo, r, e), start_hi);
            acc + format!(
                "{:02x} {}",
                mem.get(addr.0, addr.1, 0),
                if e == 7 { "- " } else { "" }
            )
            .as_str()
        });
        let addr = (safe_incr(start_lo, r, 0), start_hi);
        println!("[{:02x}:{:02x}] {line}", addr.1, addr.0)
    }
}

pub fn disassemble(cpu: &cpu::MCS6502, mem: &mem::Memory, start: &Option<String>) {
    let mut pc = match start {
        Some(start) => {
            let start = u128::from_str_radix(start, 16).unwrap_or_default();
            cmn::addr_u16_to_u8(start as u16)
        }
        None => cpu.pc(),
    };

    let mut instr_len = 0u8;
    for _ in 0..16 {
        pc = cmn::addr_u16_to_u8(cmn::offset_addr(pc.0, pc.1, instr_len));
        let (opc, bytes_str, instr_str, _, addr_mode) =
            disassemble_one_instruction(pc.0, pc.1, mem);
        instr_len = hw_dbg::ALL_OPCODE_INFO[opc as usize].bytes;
        println!(
            "{} | {} | {: >2x} │ {: >7}",
            bytes_str, instr_str, instr_len, addr_mode
        )
    }
}

fn disassemble_one_instruction(
    start_lo: u8,
    start_hi: u8,
    mem: &mem::Memory,
) -> (u8, String, String, u8, &str) {
    let opc = mem.get(start_lo, start_hi, 0);
    let opc_info = &hw_dbg::ALL_OPCODE_INFO[opc as usize];
    let instr_b1_str = if opc_info.bytes > 1 {
        &format!("{:02x}", mem.get(start_lo, start_hi, 1))
    } else {
        ""
    };
    let instr_b2_str = if opc_info.bytes > 2 {
        &format!("{:02x}", mem.get(start_lo, start_hi, 2))
    } else {
        ""
    };

    let bytes_str = format!(
        "{:02x}{:02x}: {:02x} {: <2} {: <2}",
        start_hi, start_lo, opc, instr_b1_str, instr_b2_str
    );

    let instr_str = format!(
        "{: <10}",
        opc_info
            .assembler
            .replace("oper", (instr_b2_str.to_string() + instr_b1_str).as_str())
    );

    (
        opc,
        bytes_str,
        instr_str,
        opc_info.bytes,
        opc_info.addressing,
    )
}

fn bit_value(cpu: &cpu::MCS6502, bit: cpu::PSR, val: &str) -> String {
    if cpu::tst_bit(cpu.p(), bit.bits()) {
        val.to_string()
    } else {
        val.to_lowercase()
    }
}
