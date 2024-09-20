use rustella::{cmn, tia, NtscAtari};
use std::{cell::RefCell, fs, path::PathBuf, rc::Rc};

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

    let tv = Rc::new(RefCell::new(tia::InMemoryTV::<
        { tia::NTSC_SCANLINES },
        { tia::NTSC_PIXELS_PER_SCANLINE },
    >::new(tia::ntsc_tv_config())));
    let mut atari = NtscAtari::new(tv.clone());
    let buffer = fs::read(bin_path).unwrap();
    atari.load_rom(0xF800u16, &buffer);

    atari.tick(1000000);

    for n in 0..41usize {
        assert!(
            tv.borrow().buffer()[n].iter().all(|&x| x == 0),
            "vsync & vblank areas should have all values colubk = 0."
        );
    }
    // TODO: This is a bug. It should be all zeros from 0..40
    for n in 41..42usize {
        assert!(
            tv.borrow().buffer()[n][0..67].iter().all(|&x| x == 0),
            "scanline {n} hblank area should have all values colubk = 0."
        );
        assert!(
            tv.borrow().buffer()[n][68..].iter().all(|&x| x == 1),
            "scanline {n} draw area should have all values colubk = 1."
        );
    }
    let mut colubk = 192;
    // TODO: This is a bug. It should be all rainbows from from 40..232
    for n in 42..234usize {
        assert!(
            tv.borrow().buffer()[n][0..67].iter().all(|&x| x == 0),
            "scanline {n} hblank area should have all values colubk = 0."
        );
        assert!(
            tv.borrow().buffer()[n][68..].iter().all(|&x| x == colubk),
            "scanline {n} draw area should have all values colubk = {colubk}."
        );
        colubk -= 1;
    }
    for n in 234..262 {
        assert!(
            tv.borrow().buffer()[n].iter().all(|&x| x == 0),
            "overscan area should have all values colubk = 0."
        );
    }

    assert_eq!(tv.borrow().frame_counter(), 55);

    assert_eq!(atari.cpu_state().pc(), cmn::LoHi(0x4D, 0xF8));
}
