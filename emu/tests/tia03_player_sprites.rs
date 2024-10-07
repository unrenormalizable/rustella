pub mod common;
use insta::*;
use rustella::{cmn::RefExtensions, tia, tia::TV, NtscAtari};

/// Test suite from https://www.youtube.com/watch?v=GObPgosXPPs&list=PLbPt2qKXQzJ8-P3Qe9lDPtxwFSdbDbcvW&index=5
#[test]
fn single_static_player() {
    common::setup_logger();
    let tv = tia::InMemoryTV::<{ tia::NTSC_SCANLINES }, { tia::NTSC_PIXELS_PER_SCANLINE }>::new(
        tia::ntsc_tv_config(),
    )
    .rc_refcell();
    let mut atari = NtscAtari::new(tv.clone());
    atari.load_rom(
        0xF000u16,
        &common::read_rom("8blit/8blit-s01e06-Ex1-First-Sprite.bin"),
    );

    atari.run_for(65895);

    assert_debug_snapshot!(common::serialize_tv_buffer(&tv.borrow().buffer()));
    assert_eq!(tv.borrow().frame_counter(), 82);
    assert_eq!(atari.cpu_state().cycles(), 175216);
    assert_eq!(atari.cpu_state().pc(), 0xF041.into());
}

#[test]
fn dual_player_fine_move() {
    common::setup_logger();
    let tv = tia::InMemoryTV::<{ tia::NTSC_SCANLINES }, { tia::NTSC_PIXELS_PER_SCANLINE }>::new(
        tia::ntsc_tv_config(),
    )
    .rc_refcell();
    let mut atari = NtscAtari::new(tv.clone());
    atari.load_rom(
        0xF000u16,
        &common::read_rom("8blit/8blit-s01e06-Ex4-Two Dimensional Sprite.bin"),
    );

    atari.run_for(150895);

    assert_debug_snapshot!(common::serialize_tv_buffer(&tv.borrow().buffer()));
    assert_eq!(tv.borrow().frame_counter(), 52);
    assert_eq!(atari.cpu_state().cycles(), 401412);
    assert_eq!(atari.cpu_state().pc(), 0xF046.into());
}
