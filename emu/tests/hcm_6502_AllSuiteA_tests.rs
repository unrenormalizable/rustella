pub mod common;
use rustella::{cmn, cmn::RefExtensions, cpu::*, riot};

/// Test suite from https://codegolf.stackexchange.com/q/12844.
#[test]
fn hcm_6502_allsuitea_tests_main() {
    common::setup_logger();
    let buffer = common::read_rom("hcm_6502_AllSuiteA.bin");
    let mut mem =
        riot::Memory::new_with_rom(&buffer, 0x4000.into(), riot::mm_6502, None, None, true);
    let rdy = cmn::LineState::High.rc_cell();
    let mut cpu = NMOS6502::new(rdy.clone(), &mem);

    loop {
        cpu.tick(&mut mem);
        if cpu.instructions() == 612 {
            break;
        }
    }

    assert_eq!(cpu.pc(), cmn::LoHi(0xC0, 0x45));
    assert_eq!(mem.get(cmn::LoHi(0x10, 0x02), 0), 0xFF);
    assert_eq!(cpu.cycles(), 1946);
    assert_eq!(cpu.a(), 0xFE);
    assert_eq!(cpu.x(), 0x0D);
    assert_eq!(cpu.y(), 0x54);
    assert_eq!(cpu.psr(), 0x81);
    assert_eq!(cpu.s(), 0x33);
}
