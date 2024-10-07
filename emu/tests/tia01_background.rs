pub mod common;
use insta::*;
use rustella::{cmn, cmn::RefExtensions, tia, tia::TV, NtscAtari};

/// Test suite from https://forums.atariage.com/blogs/entry/11109-step-1-generate-a-stable-display/
#[test]
fn spiceware_collect_1_stable_display() {
    common::setup_logger();

    let tv = tia::InMemoryTV::<{ tia::NTSC_SCANLINES }, { tia::NTSC_PIXELS_PER_SCANLINE }>::new(
        tia::ntsc_tv_config(),
    )
    .rc_refcell();
    let mut atari = NtscAtari::new(tv.clone());
    atari.load_rom(
        0xF800u16,
        &common::read_rom("collect/collect-01-StableDisplay.bin"),
    );

    atari.run_for(53681);

    assert_debug_snapshot!(common::serialize_tv_buffer(&tv.borrow().buffer()));
    assert_eq!(tv.borrow().frame_counter(), 54);
    assert_eq!(atari.cpu_state().cycles(), 147605);
    assert_eq!(atari.cpu_state().pc(), cmn::LoHi(0x3D, 0xF8));
}
