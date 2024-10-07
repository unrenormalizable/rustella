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

// TODO: for debug pass PC to writes

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

    registers: [u8; cmn::TIA_MAX_ADDRESS + 1],
    hsync_counter: usize,
    player_hpos_counters: [usize; 2],
    player_hpos_counters_for_next_scanline: [Option<usize>; 2],
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
            player_hpos_counters: [0, 0],
            player_hpos_counters_for_next_scanline: [None, None],
        }
    }

    fn one_tick(&mut self) {
        if self.hsync_counter == 0 {
            self.rdy.set(LineState::High);
            (0..2).for_each(|x| {
                self.player_hpos_counters_for_next_scanline[x] =
                    self.player_hpos_counters_for_next_scanline[x].and_then(|val| {
                        self.player_hpos_counters[x] = val;
                        None
                    });
            })
        }

        let color = if bits::tst_bits(self.registers[cmn::regs::VBLANK], bits::BIT_D1)
            || self.is_on_hblank()
        {
            0x00
        } else {
            grp::get_color(
                self.player_hpos_counters()[0],
                self.registers[cmn::regs::GRP0],
                self.registers[cmn::regs::COLUP0],
                self.registers[cmn::regs::REFP0],
            )
            .or_else(|| {
                grp::get_color(
                    self.player_hpos_counters()[1],
                    self.registers[cmn::regs::GRP1],
                    self.registers[cmn::regs::COLUP1],
                    self.registers[cmn::regs::REFP1],
                )
            })
            .or_else(|| pf::get_color(self.hsync_counter, &self.registers, &self.tv_cfg))
            .unwrap_or(self.registers[cmn::regs::COLUBK])
        };

        self.tv.borrow_mut().render_pixel(color);

        if !self.is_on_hblank() {
            (0..2).for_each(|x| {
                self.player_hpos_counters[x] =
                    (self.player_hpos_counters[x] + 1) % self.tv_cfg.visible_pixels();
            });
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
    pub fn player_hpos_counters(&mut self) -> &mut [usize; 2] {
        &mut self.player_hpos_counters
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn check_read_unsupported_register_flags(&self, addr: usize) {
        let (_, _, name, _) = cmn::regs::IMPLEMENTED_REGISTERS[addr];
        log::error!("Read for {name} ({addr:02X}) is not implemented yet.")
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn check_write_unsupported_register_flags(&self, addr: usize, val: u8) {
        let (w, _, name, supported_write_mask) = cmn::regs::IMPLEMENTED_REGISTERS[addr];

        // NOTE: CLEAN_START macro sets everything to 0x00, dont log for those.
        if !w && val != 0x00 {
            log::error!("{name} ({addr:02X}) is not implemented yet, Value 0b{val:08b}.");
        }

        if val & !supported_write_mask != 0 {
            log::error!("{name} ({addr:02X}) for value 0b{val:08b} is not implemented yet. Supported bits 0b{supported_write_mask:08b}.");
        }
    }

    fn initialize_player_hpos_counter(&mut self, player_id: usize) {
        if self.is_on_hblank() {
            self.player_hpos_counters[player_id] = self.tv_cfg.visible_pixels() - 3;
        } else {
            self.player_hpos_counters_for_next_scanline[player_id] = Some(
                self.tv_cfg.visible_pixels()
                    - (self.hsync_counter - self.tv_cfg.hblank_pixels() + 5)
                        % self.tv_cfg.visible_pixels(),
            );
        };
    }

    fn update_player_hpos_counter(&mut self, player_id: usize) {
        let temp = self.tv_cfg.visible_pixels() + self.player_hpos_counters[player_id];
        let delta = (self.registers[cmn::regs::HMP0 + player_id] as i8 as isize) / 0x10;
        self.player_hpos_counters[player_id] =
            temp.wrapping_add_signed(delta) % self.tv_cfg.visible_pixels();
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
                    self.tv.borrow_mut().vsync_start();
                } else {
                    self.tv.borrow_mut().vsync_end();
                }
            }

            cmn::regs::RESP0..=cmn::regs::RESP1 => {
                self.initialize_player_hpos_counter(addr - cmn::regs::RESP0);
            }

            cmn::regs::HMOVE => (0..2).for_each(|x| {
                self.update_player_hpos_counter(x);
            }),

            cmn::regs::HMCLR => {
                self.registers[cmn::regs::HMP0] = 0;
                self.registers[cmn::regs::HMP1] = 0;
                self.registers[cmn::regs::HMM0] = 0;
                self.registers[cmn::regs::HMM1] = 0;
                self.registers[cmn::regs::HMBL] = 0;
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
    use core::cell::Cell;
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
        tia.write(cmn::regs::VSYNC, bits::BIT_00);

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
        TVConfig::<5, 3>::new(2, [0x00; 128])
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
    fn playern_hpos_counter_no_reset(color_clks_done: usize, expected: usize) {
        let tv_cfg = cmn::ntsc_tv_config();
        let tv = cmn::NtscTV::new_testable(3, 0, tv_cfg);
        let mut tia = InMemoryTIA::new(LineState::Low.rc_cell(), tv.rc_refcell());

        tia.tick(color_clks_done);
        assert_eq!(
            tia.hsync_counter(),
            color_clks_done % tv_cfg.pixels_per_scanline()
        );
        assert_eq!(tia.player_hpos_counters()[0], expected);
        assert_eq!(tia.player_hpos_counters()[1], expected);
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
        let tia = InMemoryTIA::new(rdy.clone(), tv.rc_refcell()).rc_refcell();

        tia.borrow_mut().write(cmn::regs::RESP0, 0x00);
        tia.borrow_mut().tick(color_clks_done);
        let prev_p0_hpos = tia.borrow_mut().player_hpos_counters()[0];
        tia.borrow_mut().write(cmn::regs::RESP0, 0x00);

        let hpos_curr = if tia.borrow().is_on_hblank() {
            expected
        } else {
            prev_p0_hpos
        };
        assert_eq!(tia.borrow_mut().player_hpos_counters()[0], hpos_curr);

        wsync(tia.clone(), rdy);
        assert_eq!(tia.borrow_mut().player_hpos_counters()[0], expected);
    }

    #[test_case(0, 50*3, 0b_0000_0000, 73; "Stay in place - 0")]
    #[test_case(1, 50*3, 0b_0000_0000, 73; "Stay in place - 1")]
    #[test_case(0, 50*3, 0b_0011_0000, 76; "Move right - 0")]
    #[test_case(1, 50*3, 0b_0011_0000, 76; "Move right - 1")]
    #[test_case(0, 50*3, 0b_1110_0000, 71; "Move left - 0")]
    #[test_case(1, 50*3, 0b_1110_0000, 71; "Move left - 1")]
    fn player_hpos_counter_with_hmove(
        player_id: usize,
        color_clks_done: usize,
        hmp0: u8,
        expected: usize,
    ) {
        let rdy = LineState::Low.rc_cell();
        let tv = cmn::NtscTV::new_testable(3, 0, cmn::ntsc_tv_config());
        let tia = InMemoryTIA::new(rdy.clone(), tv.rc_refcell()).rc_refcell();

        tia.borrow_mut().tick(color_clks_done);
        tia.borrow_mut().write(cmn::regs::RESP0 + player_id, 0x00);
        wsync(tia.clone(), rdy);
        tia.borrow_mut().write(cmn::regs::HMP0 + player_id, hmp0);
        tia.borrow_mut().write(cmn::regs::HMOVE, 0x00);
        assert_eq!(tia.borrow_mut().player_hpos_counters()[player_id], expected);
    }

    fn wsync(tia: Rc<RefCell<dyn TIA>>, rdy: Rc<Cell<LineState>>) {
        tia.borrow_mut().write(cmn::regs::WSYNC, 0x00);
        while rdy.get() == LineState::Low {
            tia.borrow_mut().tick(1)
        }
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

        let half_screen = tv_cfg.visible_pixels() / 2;
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

        #[test_case(
            bits::BIT_00,
            vec![None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A)],
            vec![None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A)];
            "Duplicated + PF color")]
        #[test_case(
            bits::BIT_D1,
            vec![None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A)],
            vec![None, Some(0x3A), None, Some(0x3A), None, Some(0x3A), None, Some(0x3A), None, Some(0x3A), None, Some(0x3A), None, Some(0x3A), None, Some(0x3A), None, Some(0x3A), None, Some(0x3A)];
            "Duplicated + Player colors")]
        #[test_case(
            bits::BIT_D0,
            vec![None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A)],
            vec![Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None, Some(0x1A), None];
            "Mirrored + PF color")]
        #[test_case(
            bits::BIT_D0 | bits::BIT_D1,
            vec![None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A), None, Some(0x2A)],
            vec![Some(0x3A), None, Some(0x3A), None, Some(0x3A), None, Some(0x3A), None, Some(0x3A), None, Some(0x3A), None, Some(0x3A), None, Some(0x3A), None, Some(0x3A), None, Some(0x3A), None];
            "Mirrored + Player colors")]
        fn test_pf_colors(
            ctrl_pf: u8,
            display_left: Vec<Option<u8>>,
            display_right: Vec<Option<u8>>,
        ) {
            let cfg = TestableTVConfig::default();
            let mut regs: [u8; cmn::TIA_MAX_ADDRESS + 1] = [0x00; cmn::TIA_MAX_ADDRESS + 1];
            regs[cmn::regs::CTRLPF] = ctrl_pf;
            regs[cmn::regs::COLUPF] = 0x1A;
            regs[cmn::regs::COLUP0] = 0x2A;
            regs[cmn::regs::COLUP1] = 0x3A;
            regs[cmn::regs::PF0] = 0xA0;
            regs[cmn::regs::PF1] = 0x55;
            regs[cmn::regs::PF2] = 0xAA;

            let left: Vec<_> = (1..21usize).map(|x| get_color(x, &regs, &cfg)).collect();
            let right: Vec<_> = (21..41usize).map(|x| get_color(x, &regs, &cfg)).collect();

            assert_eq!(left, display_left);
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

    static POSITION_MASK: &[u8; 8] = &[
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
    pub fn get_color(playern_hpos_counter: usize, grpn: u8, colupn: u8, refpn: u8) -> Option<u8> {
        if playern_hpos_counter > 7 {
            return None;
        }

        let mut mask_index = 7 - playern_hpos_counter;
        if bits::tst_bits(refpn, bits::BIT_D3) {
            mask_index = playern_hpos_counter;
        }

        let mask = POSITION_MASK[mask_index];
        if bits::tst_bits(grpn, mask) {
            return Some(colupn);
        }

        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use alloc::vec;
        use alloc::vec::*;
        use test_case::test_case;

        #[test_case(0, false)]
        #[test_case(0xFE, false)]
        #[test_case(0x11, true)]
        fn get_color_transcribes_player0_hpos(color: u8, rev: bool) {
            let refpn = if rev { bits::BIT_D3 } else { bits::BIT_00 };

            let mut col: Vec<_> = (159..)
                .take(10)
                .map(|x| x % 160)
                .map(|hpos| get_color(hpos, 0b_10101111, color, refpn))
                .collect();
            if rev {
                col.reverse();
            }

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
