use crate::{
    bits,
    cmn::{Line, LineState},
    riot::MemorySegment,
    tia::{cmn, tv::TV},
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
    /// 0..(PIXELS_PER_SCANLINE / 4)
    hcount: usize,
    registers: [u8; cmn::TIA_MAX_ADDRESS + 1],
    tv: Rc<RefCell<dyn TV<SCANLINES, PIXELS_PER_SCANLINE>>>,
    rdy: Line,
}

#[allow(dead_code)]
impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize>
    InMemoryTIA<SCANLINES, PIXELS_PER_SCANLINE>
{
    pub fn new(rdy: Line, tv: Rc<RefCell<dyn TV<SCANLINES, PIXELS_PER_SCANLINE>>>) -> Self {
        Self {
            clk: 0,
            hcount: 0,
            registers: [0x00; cmn::TIA_MAX_ADDRESS + 1],
            tv,
            rdy,
        }
    }

    fn tick_core(&mut self) {
        let color = if bits::tst_bits(self.registers[cmn::read_regs::VBLANK], bits::BIT_D1) {
            0x00
        } else {
            self.registers[cmn::read_regs::COLUBK]
        };
        self.tv.borrow_mut().render_pixel(color);

        if self.clk == PIXELS_PER_SCANLINE - 1 {
            self.rdy.set(LineState::High);
        }
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn check_unsupported_register_flags(&self, addr: usize, val: u8) {
        let (supported, name) = cmn::read_regs::IMPLEMENTED_REGISTERS[addr];

        if let cmn::read_regs::VBLANK = addr {
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
    fn tick(&mut self, cycles: usize) {
        for _ in 0..cycles {
            self.tick_core();

            self.clk = (self.clk + 1) % self.tv.borrow().config().pixels_per_scanline();
            self.hcount = self.clk / 4;
        }
    }
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> MemorySegment
    for InMemoryTIA<SCANLINES, PIXELS_PER_SCANLINE>
{
    fn read(&self, addr: usize) -> u8 {
        let (_, name) = cmn::read_regs::IMPLEMENTED_REGISTERS[addr];
        todo!("Read for {name} ({addr:02X}) is not implemented yet.")
    }

    fn write(&mut self, reg: usize, val: u8) {
        #[cfg(debug_assertions)]
        self.check_unsupported_register_flags(reg, val);

        if let cmn::read_regs::WSYNC = reg {
            self.rdy.set(LineState::Low);
        }

        if let cmn::read_regs::VSYNC = reg {
            if bits::tst_bits(val, bits::BIT_D1) {
                self.tv.borrow_mut().vsync();
            }
        }

        self.registers[reg] = val;
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
        tia.write(cmn::read_regs::WSYNC, 0x00);
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

        tia.write(cmn::read_regs::VBLANK, bits::BIT_D1);
        tia.write(cmn::read_regs::COLUBK, 0xFF);
        tia.write(cmn::read_regs::VSYNC, bits::BIT_D1);
        assert_eq!(rdy.get(), LineState::Low);

        // VSYNC
        tia.write(cmn::read_regs::WSYNC, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|i| {
            assert_eq!(rdy.get(), LineState::Low, "{i}");
            tia.tick(1);
        });
        assert_eq!(rdy.get(), LineState::High);

        // VBLANK
        tia.write(cmn::read_regs::WSYNC, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|i| {
            assert_eq!(rdy.get(), LineState::Low, "{i}");
            tia.tick(1);
        });
        assert_eq!(rdy.get(), LineState::High);

        // Draw - 0
        tia.write(cmn::read_regs::VBLANK, 0x00);
        tia.write(cmn::read_regs::COLUBK, 0x10);
        tia.write(cmn::read_regs::WSYNC, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|i| {
            assert_eq!(rdy.get(), LineState::Low, "{i}");
            tia.tick(1);
        });
        assert_eq!(rdy.get(), LineState::High);

        // Draw - 1
        tia.write(cmn::read_regs::COLUBK, 0x20);
        tia.write(cmn::read_regs::WSYNC, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|_| {
            assert_eq!(rdy.get(), LineState::Low);
            tia.tick(1);
        });

        // Overscan
        tia.write(cmn::read_regs::VBLANK, bits::BIT_D1);
        tia.write(cmn::read_regs::COLUBK, 0xEE);
        tia.write(cmn::read_regs::WSYNC, 0x00);
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

    #[test_case(1, 2)]
    fn test_vsync(scanline: usize, pixel: usize) {
        let rdy = Rc::new(Cell::new(LineState::Low));
        let cfg = TestableTVConfig::default();
        let tv = Rc::new(RefCell::new(TestableTV::new_testable(scanline, pixel, cfg)));
        let mut tia = InMemoryTIA::new(rdy.clone(), tv.clone());

        tia.write(cmn::read_regs::COLUBK, 0x03);
        tia.tick(1);
        tia.write(cmn::read_regs::VSYNC, bits::BIT_D1);
        (0..cfg.pixels_per_scanline()).for_each(|_| tia.tick(1));
        tia.write(cmn::read_regs::VSYNC, 0x00);
        tia.write(cmn::read_regs::COLUBK, 0x02);
        tia.tick(1);
        tia.tick(1);

        assert_eq!(tv.borrow().buffer()[0], [0x00, 0x00, 0x00]);
        assert_eq!(tv.borrow().buffer()[1], [0x00, 0x02, 0x03]);
        assert_eq!(tv.borrow().buffer()[2], [0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_vblank() {
        let rdy = Rc::new(Cell::new(LineState::Low));
        let cfg = solid_display_config();
        let tv = Rc::new(RefCell::new(InMemoryTV::<5, 3>::new_testable(0, 0, cfg)));
        let mut tia = InMemoryTIA::new(rdy.clone(), tv.clone());

        (0..cfg.pixels_per_scanline()).for_each(|_| tia.tick(1));
        tia.write(cmn::read_regs::COLUBK, 0x03);
        tia.write(cmn::read_regs::VBLANK, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|_| tia.tick(1));
        tia.write(cmn::read_regs::COLUBK, 0x02);
        tia.write(cmn::read_regs::VBLANK, bits::BIT_D1);
        (0..cfg.pixels_per_scanline()).for_each(|_| tia.tick(1));
        tia.write(cmn::read_regs::COLUBK, 0x02);
        tia.write(cmn::read_regs::VBLANK, 0x00);
        (0..cfg.pixels_per_scanline()).for_each(|_| tia.tick(1));

        assert_eq!(tv.borrow().buffer()[1], [0x00, 0x03, 0x03]);
        assert_eq!(tv.borrow().buffer()[2], [0x00, 0x00, 0x00]);
        assert_eq!(tv.borrow().buffer()[3], [0x00, 0x02, 0x02]);
    }

    fn solid_display_config() -> TVConfig<5, 3> {
        TVConfig::<5, 3>::new(1, 1, 2, 1, [0x00; 256])
    }
}
