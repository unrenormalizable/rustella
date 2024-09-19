use rustella::{cmn, NtscAtari};
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

    let mut atari = NtscAtari::default();
    let buffer = fs::read(bin_path).unwrap();
    atari.load_rom(0xF800u16, &buffer);

    for _ in 0..1000000 {
        atari.tick();
    }

    (0..41usize).for_each(|n| {
        assert!(
            atari.tv_screen_state()[n].iter().all(|&x| x == 0),
            "vsync & vblank areas should have all values colubk = 0."
        );
    });
    // TODO: This is a bug. It should be all zeros from 0..40
    (41..42usize).for_each(|n| {
        assert!(
            atari.tv_screen_state()[n][0..67].iter().all(|&x| x == 0),
            "scanline {n} hblank area should have all values colubk = 0."
        );
        assert!(
            atari.tv_screen_state()[n][68..].iter().all(|&x| x == 1),
            "scanline {n} draw area should have all values colubk = 1."
        );
    });
    let mut colubk = 192;
    // TODO: This is a bug. It should be all rainbows from from 40..232
    (42..234usize).for_each(|n| {
        assert!(
            atari.tv_screen_state()[n][0..67].iter().all(|&x| x == 0),
            "scanline {n} hblank area should have all values colubk = 0."
        );
        assert!(
            atari.tv_screen_state()[n][68..]
                .iter()
                .all(|&x| x == colubk),
            "scanline {n} draw area should have all values colubk = {colubk}."
        );
        colubk -= 1;
    });
    (234..262).for_each(|n| {
        assert!(
            atari.tv_screen_state()[n].iter().all(|&x| x == 0),
            "overscan area should have all values colubk = 0."
        );
    });

    assert_eq!(atari.cpu_state().pc(), cmn::LoHi(0x4D, 0xF8));
}
