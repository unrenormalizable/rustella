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
/// - module README.md
/// - https://www.atarihq.com/danb/files/TIA_HW_Notes.txt
pub trait TIA: MemorySegment {
    fn tick(&mut self, cycles: usize);
}

pub struct InMemoryTIA<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> {
    /// 0..PIXELS_PER_SCANLINE
    clk: usize,
    registers: [u8; cmn::TIA_MAX_ADDRESS + 1],
    tv: Rc<RefCell<dyn TV<SCANLINES, PIXELS_PER_SCANLINE>>>,
    tv_cfg: TVConfig<SCANLINES, PIXELS_PER_SCANLINE>,
    rdy: Line,
}

#[allow(dead_code)]
impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize>
    InMemoryTIA<SCANLINES, PIXELS_PER_SCANLINE>
{
    pub fn new(rdy: Line, tv: Rc<RefCell<dyn TV<SCANLINES, PIXELS_PER_SCANLINE>>>) -> Self {
        let tv_cfg = *tv.borrow().config();
        Self {
            clk: 0,
            registers: [0x00; cmn::TIA_MAX_ADDRESS + 1],
            tv,
            tv_cfg,
            rdy,
        }
    }

    fn tick_core(&mut self) {
        let color = if bits::tst_bits(self.registers[cmn::regs::VBLANK], bits::BIT_D1)
            || self.clk < self.tv_cfg.hblank_pixels()
        {
            0x00
        } else {
            pf::get_color(self.clk, &self.registers, &self.tv_cfg)
                .unwrap_or(self.registers[cmn::regs::COLUBK])
        };

        self.tv.borrow_mut().render_pixel(color);

        if self.clk == PIXELS_PER_SCANLINE - 1 {
            self.rdy.set(LineState::High);
        }
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn check_read_unsupported_register_flags(&self, addr: usize) {
        let (_, _, name, _) = cmn::regs::IMPLEMENTED_REGISTERS[addr];
        todo!("Read for {name} ({addr:02X}) is not implemented yet.")
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
            self.tick_core();

            self.clk = (self.clk + 1) % self.tv.borrow().config().pixels_per_scanline();
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

        if let cmn::regs::WSYNC = addr {
            self.rdy.set(LineState::Low);
        }

        if let cmn::regs::VSYNC = addr {
            if bits::tst_bits(val, bits::BIT_D1) {
                self.tv.borrow_mut().vsync();
            }
        }

        self.registers[addr] = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tia::tv::*;
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
    type TestableTVConfig = TVConfig<SCANLINES, PIXELS_PER_SCANLINE>;

    type TestableTV = InMemoryTV<SCANLINES, PIXELS_PER_SCANLINE>;

    #[test_case(0, 0, 3)]
    #[test_case(1, 1, 2)]
    #[test_case(2, 2, 1)]
    fn test_wsync(scanline: usize, pixel: usize, ticks: usize) {
        let rdy = Rc::new(Cell::new(LineState::Low));
        let tv = TestableTV::new_testable(scanline, pixel, TestableTVConfig::default());
        let mut tia = InMemoryTIA::new(rdy.clone(), Rc::new(RefCell::new(tv)));

        assert_eq!(rdy.get(), LineState::Low);
        tia.tick(1);
        assert_eq!(rdy.get(), LineState::Low);
        tia.write(cmn::regs::WSYNC, 0x00);
        assert_eq!(rdy.get(), LineState::Low);
        (0..=ticks).for_each(|_| tia.tick(1));
        assert_eq!(rdy.get(), LineState::High);
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
        let rdy = Rc::new(Cell::new(LineState::Low));
        let cfg = solid_display_config();
        let tv = Rc::new(RefCell::new(InMemoryTV::<5, 3>::new_testable(0, 0, cfg)));
        let mut tia = InMemoryTIA::new(rdy.clone(), tv.clone());

        tia.write(cmn::regs::VBLANK, bits::BIT_D1);
        tia.write(cmn::regs::COLUBK, 0xFF);
        tia.write(cmn::regs::VSYNC, bits::BIT_D1);
        assert_eq!(rdy.get(), LineState::Low);

        // VSYNC
        tia.write(cmn::regs::WSYNC, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|i| {
            assert_eq!(rdy.get(), LineState::Low, "{i}");
            tia.tick(1);
        });
        assert_eq!(rdy.get(), LineState::High);

        // VBLANK
        tia.write(cmn::regs::WSYNC, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|i| {
            assert_eq!(rdy.get(), LineState::Low, "{i}");
            tia.tick(1);
        });
        assert_eq!(rdy.get(), LineState::High);

        // Draw - 0
        tia.write(cmn::regs::VBLANK, 0x00);
        tia.write(cmn::regs::COLUBK, 0x10);
        tia.write(cmn::regs::WSYNC, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|i| {
            assert_eq!(rdy.get(), LineState::Low, "{i}");
            tia.tick(1);
        });
        assert_eq!(rdy.get(), LineState::High);

        // Draw - 1
        tia.write(cmn::regs::COLUBK, 0x20);
        tia.write(cmn::regs::WSYNC, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|_| {
            assert_eq!(rdy.get(), LineState::Low);
            tia.tick(1);
        });

        // Overscan
        tia.write(cmn::regs::VBLANK, bits::BIT_D1);
        tia.write(cmn::regs::COLUBK, 0xEE);
        tia.write(cmn::regs::WSYNC, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|_| {
            assert_eq!(rdy.get(), LineState::Low);
            tia.tick(1);
        });
        assert_eq!(rdy.get(), LineState::High);

        assert_eq!(tv.borrow().buffer()[0], [0x00, 0x00, 0x00]);
        assert_eq!(tv.borrow().buffer()[1], [0x00, 0x00, 0x00]);
        assert_eq!(tv.borrow().buffer()[2], [0x00, 0x10, 0x10]);
        assert_eq!(tv.borrow().buffer()[3], [0x00, 0x20, 0x20]);
        assert_eq!(tv.borrow().buffer()[4], [0x00, 0x00, 0x00]);
    }

    #[test_case(1, 0)]
    fn test_vsync(scanline: usize, pixel: usize) {
        let rdy = Rc::new(Cell::new(LineState::Low));
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
        let rdy = Rc::new(Cell::new(LineState::Low));
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
