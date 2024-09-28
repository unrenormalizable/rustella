mod common;

use insta::*;
use rustella::{cmn, tia, tia::TV, NtscAtari};
use std::{cell::RefCell, fs, path::PathBuf, rc::Rc};

/// Test suite from https://forums.atariage.com/blogs/entry/11118-step-3-score-timer-display/
#[test]
fn spiceware_collect_3_score_timer_display() {
    let bin_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "bins",
        "collect",
        "collect_3.bin",
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

    loop {
        atari.tick(1);
        if atari.cpu_state().instructions() == 65895 {
            break;
        }
    }

    assert_debug_snapshot!(common::serialize_tv_buffer(&tv.borrow().buffer()));
    assert_eq!(tv.borrow().frame_counter(), 55);
    assert_eq!(atari.cpu_state().cycles(), 196871);
    assert_eq!(atari.cpu_state().pc(), cmn::LoHi(0x9D, 0xF8));
}
