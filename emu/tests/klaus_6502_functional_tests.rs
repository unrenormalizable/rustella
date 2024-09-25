use rustella::{cmn, cpu, riot};
use std::{cell::Cell, fs, path::PathBuf, rc::Rc};

/* NOTE: This is not complete yet. ADC/SBC bin & dec part of the tests are not done. */

/// Test suite from https://github.com/Klaus2m5/6502_65C02_functional_tests.
#[test]
fn klaus_6502_65c02_functional_tests_main() {
    let bin_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "klaus_6502_functional_test.bin",
    ]
    .iter()
    .collect();

    let buffer = fs::read(bin_path).unwrap();
    let mut mem =
        riot::Memory::new_with_rom(&buffer, 0x0000.into(), riot::mm_6502, None, None, true);
    let rdy = Rc::new(Cell::new(cmn::LineState::High));
    let mut cpu = cpu::MOS6502::new(rdy.clone(), &mem);
    cpu.set_pc(cmn::LoHi(0x00, 0x04));

    for _ in 0..54483 {
        cpu.tick(&mut mem);
    }

    assert_eq!(mem.get(cmn::LoHi(0x00, 0x02), 0), 0x29); // NOTE: This indicates the number of tests ran.

    assert_eq!(cpu.pc(), cmn::LoHi(0x08, 0x33));
    assert_eq!(cpu.a(), 0x29);
    assert_eq!(cpu.x(), 0xFE);
    assert_eq!(cpu.y(), 0xFF);
    assert_eq!(cpu.psr(), 0x49);
    assert_eq!(cpu.s(), 0xFF);
}
