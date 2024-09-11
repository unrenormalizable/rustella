use a2600::{cmn, cpu::*, mem};
use std::{fs, path::PathBuf};

/// Test suite from https://codegolf.stackexchange.com/q/12844.
#[test]
fn hcm_6502_allsuitea_tests_main() {
    let bin_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "hcm_6502_AllSuiteA.bin",
    ]
    .iter()
    .collect();

    let buffer = fs::read(bin_path).unwrap();
    let mut mem = mem::Memory::new_with_rom(&buffer, cmn::LoHi(0x00, 0x40), mem::mm_6502, true);
    let mut cpu = MOS6502::new(&mem);

    for _ in 0..615 {
        cpu.fetch_decode_execute(&mut mem);
    }

    assert_eq!(cpu.pc(), cmn::LoHi(0xC0, 0x45));
    assert_eq!(cpu.a(), 0xFE);
    assert_eq!(cpu.x(), 0x0D);
    assert_eq!(cpu.y(), 0x54);
    assert_eq!(cpu.psr(), 0x81);
    assert_eq!(cpu.s(), 0x33);
    assert_eq!(mem.get(cmn::LoHi(0x10, 0x02), 0), 0xFF);
}
