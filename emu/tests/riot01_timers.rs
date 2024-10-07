pub mod common;
use insta::*;
use rustella::{cmn, cmn::RefExtensions, tia, tia::TV, NtscAtari};

/// Test suite from https://forums.atariage.com/blogs/entry/11112-step-2-timers/
#[test]
fn spiceware_collect_2_timers() {
    common::setup_logger();

    let tv = tia::InMemoryTV::<{ tia::NTSC_SCANLINES }, { tia::NTSC_PIXELS_PER_SCANLINE }>::new(
        tia::ntsc_tv_config(),
    )
    .rc_refcell();
    let mut atari = NtscAtari::new(tv.clone());
    atari.load_rom(0xF800u16, &common::read_rom("collect/collect-02-Timer.bin"));

    atari.run_for(54143);

    assert_debug_snapshot!(common::serialize_tv_buffer(&tv.borrow().buffer()));
    assert_eq!(tv.borrow().frame_counter(), 62);
    assert_eq!(atari.cpu_state().cycles(), 152355);
    assert_eq!(atari.cpu_state().pc(), cmn::LoHi(0x4F, 0xF8));
}
