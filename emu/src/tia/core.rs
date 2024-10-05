use crate::{
    bits,
    cmn::{Line, LineState},
    riot::MemorySegment,
    tia::{
        cmn,
        tv::{TVConfig, TV},
    },
};
use alloc::rc::Rc;
use core::cell::RefCell;

/// Refer:
/// - https://www.atarihq.com/danb/files/TIA_HW_Notes.txt
/// - module README.md
pub trait TIA: MemorySegment {
    fn tick(&mut self, cycles: usize);
}

pub struct InMemoryTIA<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> {
    rdy: Line,
    tv: Rc<RefCell<dyn TV<SCANLINES, PIXELS_PER_SCANLINE>>>,
    tv_cfg: TVConfig<SCANLINES, PIXELS_PER_SCANLINE>,

    /// 0..PIXELS_PER_SCANLINE
    registers: [u8; cmn::TIA_MAX_ADDRESS + 1],
    hsync_counter: usize,
    player0_hpos_counter: usize,
    player0_hpos_counter_for_next_scanline: Option<usize>,
}

#[allow(dead_code)]
impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize>
    InMemoryTIA<SCANLINES, PIXELS_PER_SCANLINE>
{
    pub fn new(rdy: Line, tv: Rc<RefCell<dyn TV<SCANLINES, PIXELS_PER_SCANLINE>>>) -> Self {
        let tv_cfg = *tv.borrow().config();
        Self {
            rdy,
            tv,
            tv_cfg,
            registers: [0x00; cmn::TIA_MAX_ADDRESS + 1],
            hsync_counter: 0,
            player0_hpos_counter: 0,
            player0_hpos_counter_for_next_scanline: None,
        }
    }

    fn one_tick(&mut self) {
        if self.hsync_counter == 0 {
            self.rdy.set(LineState::High);
            self.player0_hpos_counter_for_next_scanline =
                self.player0_hpos_counter_for_next_scanline.and_then(|x| {
                    self.player0_hpos_counter = x;
                    None
                });
        }

        let color = if bits::tst_bits(self.registers[cmn::regs::VBLANK], bits::BIT_D1)
            || self.is_on_hblank()
        {
            0x00
        } else {
            grp::get_color(self.player0_hpos_counter, &self.registers)
                .or_else(|| pf::get_color(self.hsync_counter, &self.registers, &self.tv_cfg))
                .unwrap_or(self.registers[cmn::regs::COLUBK])
        };

        self.tv.borrow_mut().render_pixel(color);

        if !self.is_on_hblank() {
            self.player0_hpos_counter = (self.player0_hpos_counter + 1) % self.tv_cfg.draw_pixels();
        }
        // NOTE: This needs to be done last to signify start of next color clk.
        self.hsync_counter = (self.hsync_counter + 1) % self.tv_cfg.pixels_per_scanline();
    }

    #[inline]
    pub fn is_on_hblank(&self) -> bool {
        self.hsync_counter < self.tv_cfg.hblank_pixels()
    }

    #[inline]
    pub fn hsync_counter(&self) -> usize {
        self.hsync_counter
    }

    #[inline]
    pub fn player0_hpos_counter(&self) -> usize {
        self.player0_hpos_counter
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn check_read_unsupported_register_flags(&self, _addr: usize) {
        // TODO: Add detection of unsupported reads.
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn check_write_unsupported_register_flags(&self, addr: usize, val: u8) {
        let (w, _, name, supported_write_mask) = cmn::regs::IMPLEMENTED_REGISTERS[addr];

        assert!(
            w || val == 0x00, // NOTE: CLEAN_START macro sets everything to 0x00.
            "{name} ({addr:02X}) is not implemented yet, Value 0b{val:08b}."
        );

        assert!(
            val & !supported_write_mask == 0,
            "{name} ({addr:02X}) for value 0b{val:08b} is not implemented yet. Supported bits 0b{supported_write_mask:08b}."
        )
    }
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> TIA
    for InMemoryTIA<SCANLINES, PIXELS_PER_SCANLINE>
{
    fn tick(&mut self, cycles: usize) {
        for _ in 0..cycles {
            self.one_tick();
        }
    }
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> MemorySegment
    for InMemoryTIA<SCANLINES, PIXELS_PER_SCANLINE>
{
    fn read(&self, _addr: usize) -> u8 {
        #[cfg(debug_assertions)]
        self.check_read_unsupported_register_flags(_addr);

        0
    }

    fn write(&mut self, addr: usize, val: u8) {
        let (_, selector, _, _) = cmn::regs::IMPLEMENTED_REGISTERS[addr];
        let val = val & selector;

        #[cfg(debug_assertions)]
        self.check_write_unsupported_register_flags(addr, val);

        match addr {
            cmn::regs::WSYNC => {
                self.rdy.set(LineState::Low);
            }

            cmn::regs::VSYNC => {
                if bits::tst_bits(val, bits::BIT_D1) {
                    self.hsync_counter = 0;
                    self.tv.borrow_mut().vsync();
                }
            }

            cmn::regs::RESP0 => {
                if self.is_on_hblank() {
                    self.player0_hpos_counter = self.tv_cfg.draw_pixels() - 3;
                } else {
                    self.player0_hpos_counter_for_next_scanline = Some(
                        self.tv_cfg.draw_pixels()
                            - (self.hsync_counter - self.tv_cfg.hblank_pixels() + 5)
                                % self.tv_cfg.draw_pixels(),
                    );
                };
            }

            _ => {}
        }

        self.registers[addr] = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cmn::{LineState, RefExtensions},
        tia::tv::*,
    };
    use test_case::test_case;

    const PIXELS_PER_SCANLINE: usize = 3;
    const SCANLINES: usize = 3;

    /// TV Layout
    ///
    ///    012   
    ///   ┌───┐  
    ///  0│xxx│  
    ///  1│x  │  
    ///  2│x  │  
    ///   └───┘            
    type TestableTV = InMemoryTV<SCANLINES, PIXELS_PER_SCANLINE>;
    type TestableTVConfig = TVConfig<SCANLINES, PIXELS_PER_SCANLINE>;

    #[test_case(0, 0, 3, LineState::High)]
    #[test_case(1, 1, 2, LineState::High)]
    #[test_case(2, 2, 1, LineState::Low)]
    fn test_wsync(scanline: usize, pixel: usize, ticks: usize, final_line_state: LineState) {
        let rdy = LineState::Low.rc_cell();
        let tv = TestableTV::new_testable(scanline, pixel, TestableTVConfig::default());
        let mut tia = InMemoryTIA::new(rdy.clone(), Rc::new(RefCell::new(tv)));

        assert_eq!(rdy.get(), LineState::Low);
        tia.tick(1);
        assert_eq!(rdy.get(), LineState::High);
        tia.write(cmn::regs::WSYNC, 0x00);
        assert_eq!(rdy.get(), LineState::Low);
        (0..=ticks).for_each(|_| tia.tick(1));
        assert_eq!(rdy.get(), final_line_state);
    }

    /// The standard solid display from any atari game writing tutorial.
    ///                     
    ///   012               
    ///  ┌───┐              
    /// 0│xxx│ -> vsync     
    /// 1│x  │ -> vblank    
    /// 2│x  │              
    /// 3│x  │              
    /// 4│x  │ -> overscan  
    ///  └───┘              
    #[test]
    fn render_solid_display() {
        let rdy = LineState::Low.rc_cell();
        let cfg = solid_display_config();
        let tv = Rc::new(RefCell::new(InMemoryTV::<5, 3>::new_testable(0, 0, cfg)));
        let mut tia = InMemoryTIA::new(rdy.clone(), tv.clone());

        tia.write(cmn::regs::VBLANK, bits::BIT_D1);
        tia.write(cmn::regs::COLUBK, 0xFF);
        tia.write(cmn::regs::VSYNC, bits::BIT_D1);
        assert_eq!(rdy.get(), LineState::Low);

        // VSYNC
        tia.write(cmn::regs::WSYNC, 0x00);
        assert_eq!(rdy.get(), LineState::Low);
        (0..cfg.pixels_per_scanline()).for_each(|_| {
            tia.tick(1);
            assert_eq!(rdy.get(), LineState::High);
        });

        // VBLANK
        tia.write(cmn::regs::WSYNC, 0x00);
        assert_eq!(rdy.get(), LineState::Low);
        (0..cfg.pixels_per_scanline()).for_each(|_| {
            tia.tick(1);
            assert_eq!(rdy.get(), LineState::High);
        });

        // Draw - 0
        tia.write(cmn::regs::VBLANK, 0x00);
        tia.write(cmn::regs::COLUBK, 0x10);
        tia.write(cmn::regs::WSYNC, 0x00);
        assert_eq!(rdy.get(), LineState::Low);
        (0..cfg.pixels_per_scanline()).for_each(|_| {
            tia.tick(1);
            assert_eq!(rdy.get(), LineState::High);
        });

        // Draw - 1
        tia.write(cmn::regs::COLUBK, 0x20);
        tia.write(cmn::regs::WSYNC, 0x00);
        assert_eq!(rdy.get(), LineState::Low);
        (0..cfg.pixels_per_scanline()).for_each(|_| {
            tia.tick(1);
            assert_eq!(rdy.get(), LineState::High);
        });

        // Overscan
        tia.write(cmn::regs::VBLANK, bits::BIT_D1);
        tia.write(cmn::regs::COLUBK, 0xEE);
        tia.write(cmn::regs::WSYNC, 0x00);
        assert_eq!(rdy.get(), LineState::Low);
        (0..cfg.pixels_per_scanline()).for_each(|_| {
            tia.tick(1);
            assert_eq!(rdy.get(), LineState::High);
        });

        assert_eq!(tv.borrow().buffer()[0], [0x00, 0x00, 0x00]);
        assert_eq!(tv.borrow().buffer()[1], [0x00, 0x00, 0x00]);
        assert_eq!(tv.borrow().buffer()[2], [0x00, 0x10, 0x10]);
        assert_eq!(tv.borrow().buffer()[3], [0x00, 0x20, 0x20]);
        assert_eq!(tv.borrow().buffer()[4], [0x00, 0x00, 0x00]);
    }

    #[test_case(1, 0)]
    fn test_vsync(scanline: usize, pixel: usize) {
        let rdy = LineState::Low.rc_cell();
        let cfg = TestableTVConfig::default();
        let tv = Rc::new(RefCell::new(TestableTV::new_testable(scanline, pixel, cfg)));
        let mut tia = InMemoryTIA::new(rdy.clone(), tv.clone());

        tia.tick(2);
        tia.write(cmn::regs::COLUBK, 0x04);
        tia.tick(1);
        tia.write(cmn::regs::VSYNC, bits::BIT_D1);
        tia.tick(cfg.pixels_per_scanline());
        tia.write(cmn::regs::VSYNC, 0x00);
        tia.write(cmn::regs::COLUBK, 0x02);
        tia.tick(1);
        tia.tick(1);

        assert_eq!(tv.borrow().buffer()[0], [0x00, 0x00, 0x00]);
        assert_eq!(tv.borrow().buffer()[1], [0x00, 0x02, 0x04]);
        assert_eq!(tv.borrow().buffer()[2], [0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_vblank() {
        let rdy = LineState::Low.rc_cell();
        let cfg = solid_display_config();
        let tv = Rc::new(RefCell::new(InMemoryTV::<5, 3>::new_testable(0, 0, cfg)));
        let mut tia = InMemoryTIA::new(rdy.clone(), tv.clone());

        (0..cfg.pixels_per_scanline()).for_each(|_| tia.tick(1));
        tia.write(cmn::regs::COLUBK, 0x04);
        tia.write(cmn::regs::VBLANK, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|_| tia.tick(1));
        tia.write(cmn::regs::COLUBK, 0x02);
        tia.write(cmn::regs::VBLANK, bits::BIT_D1);
        (0..cfg.pixels_per_scanline()).for_each(|_| tia.tick(1));
        tia.write(cmn::regs::COLUBK, 0x02);
        tia.write(cmn::regs::VBLANK, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|_| tia.tick(1));

        assert_eq!(tv.borrow().buffer()[1], [0x00, 0x04, 0x04]);
        assert_eq!(tv.borrow().buffer()[2], [0x00, 0x00, 0x00]);
        assert_eq!(tv.borrow().buffer()[3], [0x00, 0x02, 0x02]);
    }

    fn solid_display_config() -> TVConfig<5, 3> {
        TVConfig::<5, 3>::new(1, 1, 2, 2, [0x00; 128])
    }

    #[test_case(0, 0; "Dormant during HBLANK - 0")]
    #[test_case(1, 0; "Dormant during HBLANK - 1")]
    #[test_case(67, 0; "Dormant during HBLANK - last")]
    #[test_case(68, 0; "Start at visible")]
    #[test_case(69, 1; "Increment at visible - 1")]
    #[test_case(227, 159; "Increment at visible - 2")]
    #[test_case(228, 0; "Wrap around - 1")]
    #[test_case(228+67, 0; "Wrap around - 2")]
    #[test_case(228+68, 0; "Wrap around - 3")]
    #[test_case(228+69, 1; "Wrap around - 4")]
    fn player0_hpos_counter_no_reset(color_clks_done: usize, expected: usize) {
        let tv_cfg = cmn::ntsc_tv_config();
        let tv = cmn::NtscTV::new_testable(3, 0, tv_cfg);
        let mut tia = InMemoryTIA::new(LineState::Low.rc_cell(), tv.rc_refcell());

        tia.tick(color_clks_done);
        assert_eq!(
            tia.hsync_counter(),
            color_clks_done % tv_cfg.pixels_per_scanline()
        );
        assert_eq!(tia.player0_hpos_counter(), expected);
    }

    #[test_case(67, 157; "At HBLANK end")]
    #[test_case(68, 155; "At visible start")]
    #[test_case(0, 157; "Hand checked with stella - 01")]
    #[test_case(3*3, 157; "Hand checked with stella - 02")]
    #[test_case(21*3, 157; "Hand checked with stella - 03")]
    #[test_case(22*3, 157; "Hand checked with stella - 04")]
    #[test_case(23*3, 154; "Hand checked with stella - 05")]
    #[test_case(24*3, 151; "Hand checked with stella - 06")]
    #[test_case(74*3, 1; "Hand checked with stella - 07")]
    #[test_case(75*3, 158; "Hand checked with stella - 08")]
    #[test_case(76*3, 157; "Hand checked with stella - 09")]
    #[test_case(77*3, 157; "Hand checked with stella - 10")]
    #[test_case(78*3, 157; "Hand checked with stella - 11")]
    #[test_case(79*3, 157; "Hand checked with stella - 12")]
    #[test_case(98*3, 157; "Hand checked with stella - 13")]
    #[test_case(99*3, 154; "Hand checked with stella - 14")]
    #[test_case(100*3, 151; "Hand checked with stella - 15")]
    #[test_case(150*3, 1; "Hand checked with stella - 16")]
    #[test_case(151*3, 158; "Hand checked with stella - 17")]
    #[test_case(152*3, 157; "Hand checked with stella - 18")]
    #[test_case(153*3, 157; "Hand checked with stella - 19")]
    #[test_case(154*3, 157; "Hand checked with stella - 20")]
    #[test_case(155*3, 157; "Hand checked with stella - 21")]
    #[test_case(174*3, 157; "Hand checked with stella - 22")]
    #[test_case(175*3, 154; "Hand checked with stella - 23")]
    #[test_case(176*3, 151; "Hand checked with stella - 24")]
    /// NOTE: These tests cases have been hand checked with stella + online sources discussing course aligning player sprites.
    fn player0_hpos_counter_with_reset(color_clks_done: usize, expected: usize) {
        let rdy = LineState::Low.rc_cell();
        let tv = cmn::NtscTV::new_testable(3, 0, cmn::ntsc_tv_config());
        let mut tia = InMemoryTIA::new(rdy.clone(), tv.rc_refcell());

        tia.write(cmn::regs::RESP0, 0x00);
        tia.tick(color_clks_done);
        let prev_p0_hpos = tia.player0_hpos_counter();
        tia.write(cmn::regs::RESP0, 0x00);

        assert_eq!(
            tia.player0_hpos_counter(),
            if tia.is_on_hblank() {
                expected
            } else {
                prev_p0_hpos
            }
        );

        tia.write(cmn::regs::WSYNC, 0x00);
        while rdy.get() == LineState::Low {
            tia.tick(1)
        }
        assert_eq!(tia.player0_hpos_counter(), expected);
    }
}

mod pf {
    use super::*;

    pub const PLAYFIELD_WIDTH: usize = 20;

    pub static PLAYFIELD_MAP: &[(usize, u8); PLAYFIELD_WIDTH] = &[
        (cmn::regs::PF0, bits::BIT_D4),
        (cmn::regs::PF0, bits::BIT_D5),
        (cmn::regs::PF0, bits::BIT_D6),
        (cmn::regs::PF0, bits::BIT_D7),
        (cmn::regs::PF1, bits::BIT_D7),
        (cmn::regs::PF1, bits::BIT_D6),
        (cmn::regs::PF1, bits::BIT_D5),
        (cmn::regs::PF1, bits::BIT_D4),
        (cmn::regs::PF1, bits::BIT_D3),
        (cmn::regs::PF1, bits::BIT_D2),
        (cmn::regs::PF1, bits::BIT_D1),
        (cmn::regs::PF1, bits::BIT_D0),
        (cmn::regs::PF2, bits::BIT_D0),
        (cmn::regs::PF2, bits::BIT_D1),
        (cmn::regs::PF2, bits::BIT_D2),
        (cmn::regs::PF2, bits::BIT_D3),
        (cmn::regs::PF2, bits::BIT_D4),
        (cmn::regs::PF2, bits::BIT_D5),
        (cmn::regs::PF2, bits::BIT_D6),
        (cmn::regs::PF2, bits::BIT_D7),
    ];

    pub fn get_color<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize>(
        clk: usize,
        registers: &[u8; cmn::TIA_MAX_ADDRESS + 1],
        tv_cfg: &TVConfig<SCANLINES, PIXELS_PER_SCANLINE>,
    ) -> Option<u8> {
        if registers[cmn::regs::PF0] == 0x00
            && registers[cmn::regs::PF1] == 0x00
            && registers[cmn::regs::PF2] == 0x00
        {
            return None;
        }

        let half_screen = tv_cfg.draw_pixels() / 2;
        let pixels_per_playfield_pixel = half_screen / PLAYFIELD_WIDTH;
        let pixel = (clk - tv_cfg.hblank_pixels()) % half_screen;
        let first_half = (clk - tv_cfg.hblank_pixels()) / half_screen == 0;

        let mut map_index = pixel / pixels_per_playfield_pixel;
        if !first_half && bits::tst_bits(registers[cmn::regs::CTRLPF], bits::BIT_D0) {
            map_index = PLAYFIELD_WIDTH - map_index - 1;
        };

        let reg_info = PLAYFIELD_MAP[map_index];
        if bits::tst_bits(registers[reg_info.0], reg_info.1) {
            if bits::tst_bits(registers[cmn::regs::CTRLPF], bits::BIT_D1) {
                if first_half {
                    Some(registers[cmn::regs::COLUP0])
                } else {
                    Some(registers[cmn::regs::COLUP1])
                }
            } else {
                Some(registers[cmn::regs::COLUPF])
            }
        } else {
            None
        }
    }

    #[cfg(test)]
    #[allow(non_snake_case)]
    mod tests {
        use super::*;
        use alloc::vec;
        use alloc::vec::*;
        use test_case::test_case;

        type TestableTVConfig = TVConfig<3, { 2 * PLAYFIELD_WIDTH + 1 }>;

        #[test]
        fn test_no_pf() {
            let cfg = TestableTVConfig::default();
            let regs: [u8; cmn::TIA_MAX_ADDRESS + 1] = [0x00; cmn::TIA_MAX_ADDRESS + 1];
            let x = get_color(68, &regs, &cfg);

            assert_eq!(x, None)
        }

        #[test_case(0xAA, (0xF0, 0xFF, 0xFF), [Some(0xAA); PLAYFIELD_WIDTH]; "Full")]
        #[test_case(0x1A, (0xA0, 0x55, 0xAA), [None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A)]; "Alternates")]
        fn test_pf_patterns(col_pf: u8, pf: (u8, u8, u8), display: [Option<u8>; PLAYFIELD_WIDTH]) {
            let cfg = TestableTVConfig::default();
            let mut regs: [u8; cmn::TIA_MAX_ADDRESS + 1] = [0x00; cmn::TIA_MAX_ADDRESS + 1];
            regs[cmn::regs::COLUPF] = col_pf;
            regs[cmn::regs::PF0] = pf.0;
            regs[cmn::regs::PF1] = pf.1;
            regs[cmn::regs::PF2] = pf.2;

            let obtained: Vec<_> = (1..21usize).map(|x| get_color(x, &regs, &cfg)).collect();

            assert_eq!(obtained, display)
        }

        #[test_case(0x1A, (0xA0, 0x55, 0xAA), bits::BIT_00, [None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A)]; "Normal")]
        #[test_case(0x55, (0xF0, 0xFC, 0x00), bits::BIT_D0, [Some(0x55), Some(0x55), Some(0x55), Some(0x55), Some(0x55), Some(0x55), Some(0x55), Some(0x55), Some(0x55), Some(0x55), None, None, None, None, None, None, None, None, None, None]; "Mirror")]
        fn test_pf_ctrlpf_D0(
            col_pf: u8,
            pf: (u8, u8, u8),
            ctrl_pf: u8,
            display: [Option<u8>; PLAYFIELD_WIDTH],
        ) {
            let cfg = TestableTVConfig::default();
            let mut regs: [u8; cmn::TIA_MAX_ADDRESS + 1] = [0x00; cmn::TIA_MAX_ADDRESS + 1];
            regs[cmn::regs::CTRLPF] = ctrl_pf;
            regs[cmn::regs::COLUPF] = col_pf;
            regs[cmn::regs::PF0] = pf.0;
            regs[cmn::regs::PF1] = pf.1;
            regs[cmn::regs::PF2] = pf.2;

            let left: Vec<_> = (1..21usize).map(|x| get_color(x, &regs, &cfg)).collect();
            let mut right: Vec<_> = (21..41usize).map(|x| get_color(x, &regs, &cfg)).collect();
            if ctrl_pf == bits::BIT_D0 {
                right.reverse()
            }

            assert_eq!(left, display);
            assert_eq!(right, display);
        }

        #[test_case((0x1A, 0x2A, 0x3A), (0xA0, 0x55, 0xAA), bits::BIT_00, vec![None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A)]; "PF color")]
        #[test_case((0x1A, 0x2A, 0x3A), (0xA0, 0x55, 0xAA), bits::BIT_D1, vec![None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A)]; "Player colors")]
        fn test_pf_ctrlpf_D1(
            cols: (u8, u8, u8),
            pf: (u8, u8, u8),
            ctrl_pf: u8,
            display: Vec<Option<u8>>,
        ) {
            let cfg = TestableTVConfig::default();
            let mut regs: [u8; cmn::TIA_MAX_ADDRESS + 1] = [0x00; cmn::TIA_MAX_ADDRESS + 1];
            regs[cmn::regs::CTRLPF] = ctrl_pf;
            regs[cmn::regs::COLUPF] = cols.0;
            regs[cmn::regs::COLUP0] = cols.1;
            regs[cmn::regs::COLUP1] = cols.2;
            regs[cmn::regs::PF0] = pf.0;
            regs[cmn::regs::PF1] = pf.1;
            regs[cmn::regs::PF2] = pf.2;

            let left: Vec<_> = (1..21usize).map(|x| get_color(x, &regs, &cfg)).collect();
            let right: Vec<_> = (21..41usize).map(|x| get_color(x, &regs, &cfg)).collect();
            let display_right = if ctrl_pf == bits::BIT_D1 {
                display
                    .clone()
                    .iter()
                    .map(|col| col.map(|_| cols.2))
                    .collect::<Vec<_>>()
            } else {
                display.clone()
            };

            assert_eq!(left, display);
            assert_eq!(right, display_right);
        }
    }
}

mod grp {
    use super::*;

    // Player registers: https://youtu.be/GObPgosXPPs?list=PLbPt2qKXQzJ8-P3Qe9lDPtxwFSdbDbcvW&t=534
    // COLUPn          => color
    // GRPn
    // RESPn -s-       => start drawing
    // REFPn BIT3      => reflect player
    // HMPn  xxxx----  => 8 px right, 8 px left
    // HMCLR -s-       => clears hmotion for p,m,b
    // HMOVE -s-       => apply fine tuning set in hmotion

    pub static POSITION_MASK: &[u8; 8] = &[
        bits::BIT_D0,
        bits::BIT_D1,
        bits::BIT_D2,
        bits::BIT_D3,
        bits::BIT_D4,
        bits::BIT_D5,
        bits::BIT_D6,
        bits::BIT_D7,
    ];

    /// Refer:
    /// - https://forums.atariage.com/topic/208473-resp0-while-pixel-position-is-negativelow/#comment-2692375
    /// - https://forums.atariage.com/topic/271085-when-do-i-use-resp0-command-to-put-sprite-on-left-of-screen/#comment-3871580
    pub fn get_color(
        player0_hpos_counter: usize,
        registers: &[u8; cmn::TIA_MAX_ADDRESS + 1],
    ) -> Option<u8> {
        if player0_hpos_counter > 7 {
            return None;
        }

        let mask = POSITION_MASK[7 - player0_hpos_counter];
        if bits::tst_bits(registers[cmn::regs::GRP0], mask) {
            return Some(registers[cmn::regs::COLUP0]);
        }

        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use alloc::vec;
        use alloc::vec::*;
        use test_case::test_case;

        #[test_case(0)]
        #[test_case(0xFE)]
        fn get_color_transcribes_player_hpos(color: u8) {
            let mut regs: [u8; cmn::TIA_MAX_ADDRESS + 1] = [0x00; cmn::TIA_MAX_ADDRESS + 1];

            regs[cmn::regs::COLUP0] = color;
            regs[cmn::regs::GRP0] = 0b_10101111;

            let col: Vec<_> = (159..)
                .take(10)
                .map(|x| x % 160)
                .map(|hpos| get_color(hpos, &regs))
                .collect();

            assert_eq!(
                col,
                vec![
                    None,
                    Some(color),
                    None,
                    Some(color),
                    None,
                    Some(color),
                    Some(color),
                    Some(color),
                    Some(color),
                    None
                ]
            )
        }
    }
}
