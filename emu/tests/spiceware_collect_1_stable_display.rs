use rustella::{cmn, cmn::RDYLine, cpu::*, mem, tia, tia::TIA};
use std::{cell::RefCell, rc::Rc};
use std::{fs, path::PathBuf};

/// Test suite from https://forums.atariage.com/blogs/entry/11109-step-1-generate-a-stable-display/
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

    let buffer = fs::read(bin_path).unwrap();
    let tv = Rc::new(RefCell::new(tia::NtscTV::new(tia::ntsc_tv_config())));
    let tia = Rc::new(RefCell::new(tia::NtscTIA::new(tv.clone())));
    let mut mem = mem::Memory::new_with_rom(
        &buffer,
        cmn::LoHi(0x00, 0xF8),
        mem::mm_6507,
        Some(tia.clone()),
        true,
    );
    let cpu = Rc::new(RefCell::new(MOS6502::new(&mem)));

    for _ in 0..1000000 {
        let cycles = cpu.borrow_mut().tick(&mut mem);
        let cycles = if cycles == 0 { 1 } else { cycles };
        for _ in 0..(cycles * 3) {
            tia.borrow_mut().tick();
        }

        cpu.borrow_mut().set_rdy(tia.borrow().rdy());
    }

    (0..41usize).for_each(|n| {
        assert!(
            tv.borrow().buffer()[n].iter().all(|&x| x == 0),
            "vsync & vblank areas should have all values colubk = 0."
        );
    });
    // TODO: This is a bug. It should be all zeros from 0..40
    (41..42usize).for_each(|n| {
        assert!(
            tv.borrow().buffer()[n][0..67].iter().all(|&x| x == 0),
            "scanline {n} hblank area should have all values colubk = 0."
        );
        assert!(
            tv.borrow().buffer()[n][68..].iter().all(|&x| x == 1),
            "scanline {n} draw area should have all values colubk = 1."
        );
    });
    let mut colubk = 192;
    // TODO: This is a bug. It should be all rainbows from from 40..232
    (42..234usize).for_each(|n| {
        assert!(
            tv.borrow().buffer()[n][0..67].iter().all(|&x| x == 0),
            "scanline {n} hblank area should have all values colubk = 0."
        );
        assert!(
            tv.borrow().buffer()[n][68..].iter().all(|&x| x == colubk),
            "scanline {n} draw area should have all values colubk = {colubk}."
        );
        colubk -= 1;
    });
    (234..262).for_each(|n| {
        assert!(
            tv.borrow().buffer()[n].iter().all(|&x| x == 0),
            "overscan area should have all values colubk = 0."
        );
    });

    assert_eq!(cpu.borrow().pc(), cmn::LoHi(0x4D, 0xF8));
}
