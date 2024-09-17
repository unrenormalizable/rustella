use crate::{
    bits,
    cmn::{LineState, RDYLine},
    mem::MemorySegment,
    tia::{cmn, tv::TV},
};
use alloc::rc::Rc;
use core::{cell::RefCell, fmt::Debug};

/// Refer:
/// - module README.md
/// - https://www.atarihq.com/danb/files/TIA_HW_Notes.txt
pub trait TIA: MemorySegment + RDYLine {
    fn tick(&mut self);
}

#[derive(Debug)]
pub struct InMemoryTIA<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> {
    /// 0..PIXELS_PER_SCANLINE
    clk: usize,
    /// 0..(PIXELS_PER_SCANLINE / 4)
    hcount: usize,
    registers: [u8; cmn::TIA_MAX_ADDRESS + 1],
    tv: Rc<RefCell<dyn TV<SCANLINES, PIXELS_PER_SCANLINE>>>,
    rdy: LineState,
}

#[allow(dead_code)]
impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize>
    InMemoryTIA<SCANLINES, PIXELS_PER_SCANLINE>
{
    pub fn new(tv: Rc<RefCell<dyn TV<SCANLINES, PIXELS_PER_SCANLINE>>>) -> Self {
        Self {
            clk: 0,
            hcount: 0,
            registers: [0x00; cmn::TIA_MAX_ADDRESS + 1],
            tv,
            rdy: LineState::Low,
        }
    }

    fn tick_core(&mut self) {
        let color = if bits::tst_bits(self.registers[cmn::regs::VBLANK], bits::BIT_D1) {
            0x00
        } else {
            self.registers[cmn::regs::COLUBK]
        };
        self.tv.borrow_mut().render_pixel(color);

        if self.clk == PIXELS_PER_SCANLINE - 1 {
            self.set_rdy(LineState::High);
        }
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn check_unsupported_register_flags(&self, addr: usize, val: u8) {
        let (supported, name) = cmn::IMPLEMENTED_REGISTERS[addr];

        if let cmn::regs::VBLANK = addr {
            assert!(
                val & !bits::BIT_D1 == 0,
                "{name} ({addr:02X}) for value 0x{addr:02X} is not implemented yet."
            )
        }

        assert!(
            val == 0x00 || supported,
            "{name} ({addr:02X}) is not implemented yet, Value 0x{val:02X}."
        )
    }
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> TIA
    for InMemoryTIA<SCANLINES, PIXELS_PER_SCANLINE>
{
    fn tick(&mut self) {
        self.tick_core();

        self.clk = (self.clk + 1) % self.tv.borrow().config().pixels_per_scanline();
        self.hcount = self.clk / 4;
    }
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> MemorySegment
    for InMemoryTIA<SCANLINES, PIXELS_PER_SCANLINE>
{
    fn read(&self, addr: usize) -> u8 {
        let (_, name) = cmn::IMPLEMENTED_REGISTERS[addr];
        todo!("Read for {name} ({addr:02X}) is not implemented yet.")
    }

    fn write(&mut self, reg: usize, val: u8) {
        #[cfg(debug_assertions)]
        self.check_unsupported_register_flags(reg, val);

        if let cmn::regs::WSYNC = reg {
            self.set_rdy(LineState::Low);
        }

        if let cmn::regs::VSYNC = reg {
            if bits::tst_bits(val, bits::BIT_D1) {
                self.tv.borrow_mut().vsync();
            }
        }

        self.registers[reg] = val;
    }
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> RDYLine
    for InMemoryTIA<SCANLINES, PIXELS_PER_SCANLINE>
{
    fn rdy(&self) -> LineState {
        self.rdy
    }

    fn set_rdy(&mut self, rdy: LineState) {
        self.rdy = rdy;
    }
}

// WSYNC, VSYNC, VBLANK, COLUBCK

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tia::tv::*;
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
        let tv = TestableTV::new_testable(scanline, pixel, TestableTVConfig::default());
        let mut tia = InMemoryTIA::new(Rc::new(RefCell::new(tv)));

        assert_eq!(tia.rdy(), LineState::Low);
        tia.tick();
        assert_eq!(tia.rdy(), LineState::Low);
        tia.write(cmn::regs::WSYNC, 0x00);
        assert_eq!(tia.rdy(), LineState::Low);
        (0..=ticks).for_each(|_| tia.tick());
        assert_eq!(tia.rdy(), LineState::High);
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
        let cfg = solid_display_config();
        let tv = Rc::new(RefCell::new(InMemoryTV::<5, 3>::new_testable(0, 0, cfg)));
        let mut tia = InMemoryTIA::new(tv.clone());

        tia.write(cmn::regs::VBLANK, bits::BIT_D1);
        tia.write(cmn::regs::COLUBK, 0xFF);
        tia.write(cmn::regs::VSYNC, bits::BIT_D1);
        assert_eq!(tia.rdy(), LineState::Low);

        // VSYNC
        tia.write(cmn::regs::WSYNC, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|i| {
            assert_eq!(tia.rdy(), LineState::Low, "{i}");
            tia.tick();
        });
        assert_eq!(tia.rdy(), LineState::High);

        // VBLANK
        tia.write(cmn::regs::WSYNC, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|i| {
            assert_eq!(tia.rdy(), LineState::Low, "{i}");
            tia.tick();
        });
        assert_eq!(tia.rdy(), LineState::High);

        // Draw - 0
        tia.write(cmn::regs::VBLANK, 0x00);
        tia.write(cmn::regs::COLUBK, 0x10);
        tia.write(cmn::regs::WSYNC, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|i| {
            assert_eq!(tia.rdy(), LineState::Low, "{i}");
            tia.tick();
        });
        assert_eq!(tia.rdy(), LineState::High);

        // Draw - 1
        tia.write(cmn::regs::COLUBK, 0x20);
        tia.write(cmn::regs::WSYNC, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|_| {
            assert_eq!(tia.rdy(), LineState::Low);
            tia.tick();
        });

        // Overscan
        tia.write(cmn::regs::VBLANK, bits::BIT_D1);
        tia.write(cmn::regs::COLUBK, 0xEE);
        tia.write(cmn::regs::WSYNC, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|_| {
            assert_eq!(tia.rdy(), LineState::Low);
            tia.tick();
        });
        assert_eq!(tia.rdy(), LineState::High);

        assert_eq!(tv.borrow().buffer()[0], [0x00, 0x00, 0x00]);
        assert_eq!(tv.borrow().buffer()[1], [0x00, 0x00, 0x00]);
        assert_eq!(tv.borrow().buffer()[2], [0x00, 0x10, 0x10]);
        assert_eq!(tv.borrow().buffer()[3], [0x00, 0x20, 0x20]);
        assert_eq!(tv.borrow().buffer()[4], [0x00, 0x00, 0x00]);
    }

    #[test_case(1, 2)]
    fn test_vsync(scanline: usize, pixel: usize) {
        let cfg = TestableTVConfig::default();
        let tv = Rc::new(RefCell::new(TestableTV::new_testable(scanline, pixel, cfg)));
        let mut tia = InMemoryTIA::new(tv.clone());

        tia.write(cmn::regs::COLUBK, 0x03);
        tia.tick();
        tia.write(cmn::regs::VSYNC, bits::BIT_D1);
        (0..cfg.pixels_per_scanline()).for_each(|_| tia.tick());
        tia.write(cmn::regs::VSYNC, 0x00);
        tia.write(cmn::regs::COLUBK, 0x02);
        tia.tick();
        tia.tick();

        assert_eq!(tv.borrow().buffer()[0], [0x00, 0x00, 0x00]);
        assert_eq!(tv.borrow().buffer()[1], [0x00, 0x02, 0x03]);
        assert_eq!(tv.borrow().buffer()[2], [0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_vblank() {
        let cfg = solid_display_config();
        let tv = Rc::new(RefCell::new(InMemoryTV::<5, 3>::new_testable(0, 0, cfg)));
        let mut tia = InMemoryTIA::new(tv.clone());

        (0..cfg.pixels_per_scanline()).for_each(|_| tia.tick());
        tia.write(cmn::regs::COLUBK, 0x03);
        tia.write(cmn::regs::VBLANK, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|_| tia.tick());
        tia.write(cmn::regs::COLUBK, 0x02);
        tia.write(cmn::regs::VBLANK, bits::BIT_D1);
        (0..cfg.pixels_per_scanline()).for_each(|_| tia.tick());
        tia.write(cmn::regs::COLUBK, 0x02);
        tia.write(cmn::regs::VBLANK, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|_| tia.tick());

        assert_eq!(tv.borrow().buffer()[1], [0x00, 0x03, 0x03]);
        assert_eq!(tv.borrow().buffer()[2], [0x00, 0x00, 0x00]);
        assert_eq!(tv.borrow().buffer()[3], [0x00, 0x02, 0x02]);
    }

    fn solid_display_config() -> TVConfig<5, 3> {
        TVConfig::<5, 3>::new(1, 1, 2, 1)
    }
}
