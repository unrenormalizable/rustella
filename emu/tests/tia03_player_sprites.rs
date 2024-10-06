mod common;

use insta::*;
use rustella::{tia, tia::TV, NtscAtari};
use std::{cell::RefCell, fs, path::PathBuf, rc::Rc};

/// Test suite from https://www.youtube.com/watch?v=GObPgosXPPs&list=PLbPt2qKXQzJ8-P3Qe9lDPtxwFSdbDbcvW&index=5
#[test]
fn single_static_player() {
    log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    let bin_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "bins",
        "8blit",
        "8blit-s01e06-Ex1-First-Sprite.bin",
    ]
    .iter()
    .collect();

    let tv = Rc::new(RefCell::new(tia::InMemoryTV::<
        { tia::NTSC_SCANLINES },
        { tia::NTSC_PIXELS_PER_SCANLINE },
    >::new(tia::ntsc_tv_config())));
    let mut atari = NtscAtari::new(tv.clone());
    let buffer = fs::read(bin_path).unwrap();
    atari.load_rom(0xF000u16, &buffer);

    loop {
        atari.tick(1);
        if atari.cpu_state().instructions() == 65895 {
            break;
        }
    }

    assert_debug_snapshot!(common::serialize_tv_buffer(&tv.borrow().buffer()));
    assert_eq!(tv.borrow().frame_counter(), 82);
    assert_eq!(atari.cpu_state().cycles(), 175216);
    assert_eq!(atari.cpu_state().pc(), 0xF041.into());
}
