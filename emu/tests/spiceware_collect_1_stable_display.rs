use a2600_emu::{cmn, cpu::*, mem, tia, tia::TIA};
use std::{cell::RefCell, rc::Rc};
use std::{fs, path::PathBuf};

/// Test suite from https://forums.atariage.com/blogs/entry/11109-step-1-generate-a-stable-display/
#[ignore]
#[test]
fn spiceware_collect_1_stable_display() {
    let bin_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "spiceware_collect",
        "1",
        "collect.bin",
    ]
    .iter()
    .collect();

    let mmap = Box::new(mem::MMap6507::default());
    let buffer = fs::read(bin_path).unwrap();
    let tv = Rc::new(RefCell::new(tia::InMemoryTV::<
        { tia::ntsc::SCAN_LINES },
        { tia::CYCLES_PER_SCAN_LINE },
    >::default()));
    let tia = Rc::new(RefCell::new(tia::InMemoryTIA::new(tv)));
    let mut mem = mem::Memory::new_with_rom(
        &buffer,
        cmn::LoHi(0x00, 0xF8),
        mmap,
        Some(tia.clone()),
        true,
    );
    let cpu = Rc::new(RefCell::new(MOS6502::new(&mem)));
    tia.borrow_mut().connect_rdy(cpu.clone());

    // Connect RDY line
    // Ticks

    for _ in 0..615 {
        cpu.borrow_mut().tick(&mut mem);
        tia.borrow_mut().tick();
    }

    assert_eq!(cpu.borrow().pc(), cmn::LoHi(0xC0, 0x45));
    assert_eq!(cpu.borrow().a(), 0xFE);
    assert_eq!(cpu.borrow().x(), 0x0D);
    assert_eq!(cpu.borrow().y(), 0x54);
    assert_eq!(cpu.borrow().psr(), 0x81);
    assert_eq!(cpu.borrow().s(), 0x33);
    assert_eq!(mem.get(cmn::LoHi(0x10, 0x02), 0), 0xFF);
}
