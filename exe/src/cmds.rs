use a2600::{am, cmn, cpu, mem, opc_info};

pub fn dump_registers(cpu: &cpu::MOS6502, mem: &mem::Memory) {
    let pc = cpu.pc();
    let (_, bytes_str, instr_str, instr_len, addr_mode) = disassemble_one_instruction(pc, mem);

    println!("┌────────────────┬────────────┬─────────────┬─────────────────┬──────────────┐");
    println!("│ PC             │ OP         │ A  X  Y  S  │ N V   B D I Z C │              │");
    println!("╞════════════════╪════════════╪═════════════╪═════════════════╪════╤═════════╡");
    println!(
        "│ {} │ {: <10} │ {:02x} {:02x} {:02x} {:02x} │ {} {}   {} {} {} {} {} │ {: >2x} │ {: >7} │",
        bytes_str,
        instr_str,
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
        instr_len,
        addr_mode,
    );
    println!("└────────────────┴────────────┴─────────────┴─────────────────┴────┴─────────┘");
}

pub fn dump_memory(mem: &mem::Memory, start: &Option<String>) {
    let cmn::LoHi(start_lo, start_hi) = parse_hex_addr_opt(start, cmn::RAM_START);

    let safe_incr = |s: u8, r: u8, o: u8| ((s as u16) + 16u16 * (r as u16) + (o as u16)) as u8;

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

pub fn disassemble(cpu: &cpu::MOS6502, mem: &mem::Memory, start: &Option<String>) {
    let mut pc = parse_hex_addr_opt(start, cmn::LoHi(cpu.pc().0, cpu.pc().1));

    let mut instr_len = 0u8;
    for _ in 0..16 {
        pc = am::utils::address_indexed(pc, instr_len);
        let (opc, bytes_str, instr_str, _, addr_mode) = disassemble_one_instruction(pc, mem);
        instr_len = opc_info::ALL[opc as usize].bytes;
        println!(
            "{} | {} | {: >2x} │ {: >7}",
            bytes_str, instr_str, instr_len, addr_mode
        )
    }
}

pub fn load(_: &cpu::MOS6502, mem: &mut mem::Memory, start: &str, path: &str) {
    let start = u16::from_str_radix(start, 16).map(am::utils::u16_to_addr);
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

pub fn set_register(cpu: &mut cpu::MOS6502, _: &mem::Memory, reg: &str, val: &str) {
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
        "pc" => cpu.set_pc(am::utils::u16_to_addr(reg_val)),
        "s" => cpu.set_s(reg_val as u8),
        "p" => cpu.set_p(reg_val as u8),
        _ => println!("Unknown register {reg}"),
    }
}

fn disassemble_one_instruction(
    start: cmn::LoHi,
    mem: &mem::Memory,
) -> (u8, String, String, u8, &str) {
    let opc = mem.get(start.0, start.1, 0);
    let opc_info = &opc_info::ALL[opc as usize];
    let instr_b1_str = if opc_info.bytes > 1 {
        &format!("{:02x}", mem.get(start.0, start.1, 1))
    } else {
        ""
    };
    let instr_b2_str = if opc_info.bytes > 2 {
        &format!("{:02x}", mem.get(start.0, start.1, 2))
    } else {
        ""
    };

    let bytes_str = format!(
        "{:02x}{:02x}: {:02x} {: <2} {: <2}",
        start.1, start.0, opc, instr_b1_str, instr_b2_str
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

fn bit_value(cpu: &cpu::MOS6502, bit: cpu::PSR) -> String {
    if cpu::tst_bit(cpu.p(), bit.bits()) {
        "+".to_string()
    } else {
        " ".to_lowercase()
    }
}

fn parse_hex_addr_opt(val: &Option<String>, default: cmn::LoHi) -> cmn::LoHi {
    val.as_ref()
        .and_then(|x| u16::from_str_radix(x, 16).ok())
        .map(am::utils::u16_to_addr)
        .unwrap_or(default)
}
