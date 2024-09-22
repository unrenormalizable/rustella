use rustella::{cmn, tia, tia::TV, NtscAtari};
use std::{cell::RefCell, fs, path::PathBuf, rc::Rc};

/// Test suite from https://forums.atariage.com/blogs/entry/11112-step-2-timers/
#[test]
fn spiceware_collect_2_timers() {
    let bin_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "spiceware_collect",
        "2",
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

    let buffer = tv.borrow().buffer();
    // TODO: This is a bug. It should be all zeros from 0..40
    for sl in buffer.iter().take(43usize) {
        assert_eq!(
            sl,
            &[0x00; tia::NTSC_PIXELS_PER_SCANLINE],
            "vsync & vblank areas should have all values colubk = 0",
        );
    }
    let mut colubk = 192;
    // TODO: This is a bug. It should be all rainbows from from 40..232
    for sl in buffer.iter().take(235usize).skip(43) {
        assert_eq!(
            &sl[0..67],
            &[0x00; 67],
            "scanline hblank area should have all values colubk = 0."
        );
        assert_eq!(
            &sl[68..],
            &[colubk & !0x1; 160],
            "scanline draw area should have all values colubk = {colubk}.",
        );
        colubk -= 1;
    }
    for sl in buffer.iter().skip(235) {
        assert_eq!(
            sl,
            &[0x00; tia::NTSC_PIXELS_PER_SCANLINE],
            "overscan area should have all values colubk = 0.",
        );
    }

    assert_eq!(tv.borrow().frame_counter(), 55);

    assert_eq!(atari.cpu_state().pc(), cmn::LoHi(0x51, 0xF8));
}
