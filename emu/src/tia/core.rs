use super::{cmn, tv};
use crate::bits;
use alloc::rc::Rc;
use core::cell::RefCell;

#[allow(clippy::upper_case_acronyms)]
pub trait RDY {
    fn state(&self) -> bool;
    fn set_state(&mut self, state: bool);
}

#[derive(Default)]
struct NopRDY {}
impl RDY for NopRDY {
    #[inline]
    fn state(&self) -> bool {
        false
    }

    #[inline]
    fn set_state(&mut self, _: bool) {}
}

pub trait TIAReaderWriter {
    fn get(&self, reg: usize) -> u8;
    fn set(&mut self, reg: usize, val: u8);
}

/// Refer:
/// - module README.md
/// - https://www.atarihq.com/danb/files/TIA_HW_Notes.txt
pub trait TIA: TIAReaderWriter {
    fn frame_counter(&self) -> usize;

    fn tick_n(&mut self, n: usize);

    fn tick(&mut self);

    fn connect_rdy(&mut self, state: Rc<RefCell<dyn RDY>>);

    #[allow(non_snake_case)]
    fn rdy(&mut self, state: bool);
}

pub struct InMemoryTIA {
    frame_counter: usize,
    frame_cycle_counter: usize,
    registers: [u8; cmn::TIA_MAX_ADDRESS],
    wsync: bool,
    rdy: Rc<RefCell<dyn RDY>>,
    tv: Rc<RefCell<dyn tv::TV>>,
}

impl InMemoryTIA {
    pub fn new(tv: Rc<RefCell<dyn tv::TV>>) -> Self {
        Self {
            frame_counter: 0,
            frame_cycle_counter: 0,
            registers: [0x00; cmn::TIA_MAX_ADDRESS],
            wsync: false,
            rdy: Rc::new(RefCell::new(NopRDY::default())),
            tv,
        }
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn check_unsupported_register_flags(&self, reg: usize, val: u8) {
        if let cmn::regs::VBLANK = reg {
            let x = val & !bits::BIT_D1;
            assert!(x == 0, "Unsupported {reg:?} <= 0x{val:02X}.")
        }

        assert!(
            cmn::IMPLEMENTED_REGISTERS[reg],
            "{reg:?} is not implemented yet."
        )
    }
}

impl TIAReaderWriter for InMemoryTIA {
    fn get(&self, reg: usize) -> u8 {
        todo!("Read register not implemented yet. {reg:?}")
    }

    fn set(&mut self, reg: usize, val: u8) {
        #[cfg(debug_assertions)]
        self.check_unsupported_register_flags(reg, val);

        if let cmn::regs::WSYNC = reg {
            self.rdy(true);
        }

        if let cmn::regs::VSYNC = reg {
            if bits::tst_bits(val, bits::BIT_D1) {
                self.frame_counter += 1;
                self.frame_cycle_counter = 0;
            }
        }

        self.registers[reg] = val;
    }
}

impl TIA for InMemoryTIA {
    /// Number of times VSYNC on is called.
    #[inline]
    fn frame_counter(&self) -> usize {
        self.frame_counter
    }

    #[inline]
    fn tick_n(&mut self, n: usize) {
        (0..n).for_each(|_| self.tick())
    }

    fn tick(&mut self) {
        self.frame_cycle_counter += 1;

        let offset = (self.frame_cycle_counter - 1) % cmn::CYCLES_PER_SCAN_LINE;
        if self.wsync && offset == 0 {
            self.rdy(false);
        }
        if offset < cmn::COL_DRAWABLE_AREA_START {
            return;
        }

        let scan_line = (self.frame_cycle_counter - 1) / cmn::CYCLES_PER_SCAN_LINE;
        if scan_line < cmn::ROW_VERTICAL_SYNC_END {
            return;
        }

        if !bits::tst_bits(self.registers[cmn::regs::VBLANK], bits::BIT_D1) {
            return;
        }

        self.tv
            .borrow_mut()
            .render(scan_line, offset, self.registers[cmn::regs::COLUBK]);
    }

    #[allow(non_snake_case)]
    fn connect_rdy(&mut self, rdy: Rc<RefCell<dyn RDY>>) {
        self.rdy = rdy.clone();
    }

    #[allow(non_snake_case)]
    #[inline]
    fn rdy(&mut self, state: bool) {
        self.wsync = state;
        self.rdy.borrow_mut().set_state(state);
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn render_empty_frame() {
        let tv =
            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::CYCLES_PER_SCAN_LINE }>::default();
        let tv = Rc::new(RefCell::new(tv));
        let mut tia = InMemoryTIA::new(tv.clone());

        tia.tick_n(cmn::ntsc::CYCLES_PER_FRAME * 2);
        assert_eq!(tia.frame_counter(), 0);

        check_display!(tv.borrow().buffer(), 0x00);
    }

    #[test]
    fn render_with_VBLANK_always_on() {
        let tv =
            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::CYCLES_PER_SCAN_LINE }>::default();
        let tv = Rc::new(RefCell::new(tv));
        let mut tia = InMemoryTIA::new(tv.clone());

        let colubk = 0x1F;
        tia.set(cmn::regs::VBLANK, bits::BIT_D1);
        tia.set(cmn::regs::COLUBK, colubk);

        tia.tick_n(cmn::ntsc::CYCLES_PER_FRAME);

        check_display!(
            tv.borrow().buffer(),
            (
                cmn::ntsc::ROW_VERTICAL_BLANK_START,
                cmn::COL_DRAWABLE_AREA_START,
                cmn::ntsc::ROW_OVERSCAN_END,
                cmn::COL_DRAWABLE_AREA_END,
            ),
            (colubk, 0x00)
        );
    }

    #[test]
    fn render_solid_display() {
        let tv =
            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::CYCLES_PER_SCAN_LINE }>::default();
        let tv = Rc::new(RefCell::new(tv));
        let mut tia = InMemoryTIA::new(tv.clone());

        tia.set(cmn::regs::VSYNC, bits::BIT_D1);
        let colubk = 0xFF;
        tia.set(cmn::regs::COLUBK, colubk);

        tia.set(cmn::regs::VBLANK, 0x00);
        tia.tick_n(cmn::ntsc::CYCLES_PER_VERTICAL_SYNC);
        tia.set(cmn::regs::VSYNC, 0x00);
        tia.tick_n(cmn::ntsc::CYCLES_PER_VERTICAL_BLANK);
        tia.set(cmn::regs::VBLANK, bits::BIT_D1);
        tia.tick_n(cmn::ntsc::CYCLES_PER_DRAWABLE_AREA_AND_HBLANK);
        tia.set(cmn::regs::VBLANK, 0x00);
        tia.tick_n(cmn::ntsc::CYCLES_PER_OVERSCAN);

        assert_eq!(tia.frame_counter(), 1);
        check_display!(
            tv.borrow().buffer(),
            (
                cmn::ntsc::ROW_DRAWABLE_AREA_START,
                cmn::COL_DRAWABLE_AREA_START,
                cmn::ntsc::ROW_DRAWABLE_AREA_END,
                cmn::COL_DRAWABLE_AREA_END
            ),
            (colubk, 0x00)
        );
    }

    #[test]
    fn with_VSYNC() {
        let tv =
            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::CYCLES_PER_SCAN_LINE }>::default();
        let tv = Rc::new(RefCell::new(tv));
        let mut tia = InMemoryTIA::new(tv.clone());

        tia.set(cmn::regs::VBLANK, bits::BIT_D1);

        tia.set(cmn::regs::COLUBK, 0x11);
        tia.tick_n(cmn::ntsc::CYCLES_PER_VERTICAL_SYNC);
        tia.tick_n(cmn::CYCLES_PER_SCAN_LINE);

        tia.set(cmn::regs::VSYNC, bits::BIT_D1);
        tia.set(cmn::regs::COLUBK, 0x22);
        tia.tick_n(cmn::ntsc::CYCLES_PER_VERTICAL_SYNC);
        tia.set(cmn::regs::VSYNC, 0x00);
        tia.tick_n(cmn::CYCLES_PER_SCAN_LINE);
        assert_eq!(tia.frame_counter(), 1);
        check_display!(
            tv.borrow().buffer(),
            (
                cmn::ntsc::ROW_VERTICAL_BLANK_START,
                cmn::COL_DRAWABLE_AREA_START,
                cmn::ntsc::ROW_VERTICAL_BLANK_START + 1,
                cmn::COL_DRAWABLE_AREA_END
            ),
            (0x22, 0x00)
        );
    }

    #[test]
    fn with_WSYNC() {
        let tv =
            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::CYCLES_PER_SCAN_LINE }>::default();
        let tv = Rc::new(RefCell::new(tv));
        let rdy = Rc::new(RefCell::new(SpyRDY::new(false)));
        let mut tia = InMemoryTIA::new(tv);
        tia.connect_rdy(rdy.clone());

        // 0th scan line.
        tia.tick();
        assert!(!rdy.borrow().state());
        tia.set(cmn::regs::WSYNC, 0x00);
        assert!(rdy.borrow().state());
        tia.tick();
        assert!(rdy.borrow().state());
        tia.tick_n(cmn::CYCLES_PER_SCAN_LINE - 3);
        assert!(rdy.borrow().state());
        tia.tick();
        assert!(rdy.borrow().state());
        // 1st scan line.
        tia.tick();
        assert!(!rdy.borrow().state());
        tia.tick_n(10 * cmn::CYCLES_PER_SCAN_LINE - 1);
        assert!(!rdy.borrow().state());
        // 11th scan line.
        tia.tick();
        assert!(!rdy.borrow().state());
        tia.set(cmn::regs::WSYNC, 0x00);
        assert!(rdy.borrow().state());
        tia.tick();
        assert!(rdy.borrow().state());
        tia.tick_n(cmn::CYCLES_PER_SCAN_LINE - 3);
        assert!(rdy.borrow().state());
        tia.tick();
        assert!(rdy.borrow().state());
        tia.tick();
        assert!(!rdy.borrow().state());
    }

    struct SpyRDY {
        state: bool,
    }

    impl SpyRDY {
        pub fn new(state: bool) -> Self {
            Self { state }
        }
    }

    impl RDY for SpyRDY {
        fn state(&self) -> bool {
            self.state
        }

        fn set_state(&mut self, state: bool) {
            self.state = !self.state;
            // NOTE: This ensures it is not called more than twice.
            assert_eq!(state, self.state);
        }
    }
}
