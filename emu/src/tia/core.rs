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
pub trait TIA: MemorySegment + RDYLine + Debug {
    fn tick(&mut self);
}

#[derive(Debug)]
pub struct InMemoryTIA {
    /// 0..228
    clk: usize,
    /// 0..57
    hcount: usize,
    registers: [u8; cmn::TIA_MAX_ADDRESS],
    tv: Rc<RefCell<dyn TV>>,
    rdy: LineState,
}

#[allow(dead_code)]
impl InMemoryTIA {
    pub fn new(tv: Rc<RefCell<dyn TV>>) -> Self {
        Self {
            clk: 0,
            hcount: 0,
            registers: [0x00; cmn::TIA_MAX_ADDRESS],
            tv,
            rdy: LineState::Low,
        }
    }

    fn tick_core(&mut self) {
        // HBLANK, VSYNC, WSYNC

        let color = if !bits::tst_bits(self.registers[cmn::regs::VBLANK], bits::BIT_D1) {
            0x00
        } else {
            self.registers[cmn::regs::COLUBK]
        };

        self.tv.borrow_mut().render_pixel(color);
    }

    fn set_rdy(&mut self, rdy: LineState) {
        self.rdy = rdy;
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

impl TIA for InMemoryTIA {
    fn tick(&mut self) {
        self.tick_core();

        self.clk = (self.clk + 1) % cmn::CYCLES_PER_SCAN_LINE;
        self.hcount = self.clk / 4;
    }
}

impl MemorySegment for InMemoryTIA {
    fn read(&self, addr: usize) -> u8 {
        let (_, name) = cmn::IMPLEMENTED_REGISTERS[addr];
        todo!("Read for {name} ({addr:02X}) is not implemented yet.")
    }

    fn write(&mut self, reg: usize, val: u8) {
        #[cfg(debug_assertions)]
        self.check_unsupported_register_flags(reg, val);

        //if let cmn::regs::WSYNC = reg {
        //    self.set_rdy(LineState::Low);
        //}

        //if let cmn::regs::VSYNC = reg {
        //    if bits::tst_bits(val, bits::BIT_D1) {
        //        self.tv.borrow_mut().vsync();
        //    }
        //}

        self.registers[reg] = val;
    }
}

impl RDYLine for InMemoryTIA {
    fn rdy(&self) -> LineState {
        self.rdy
    }
}

//impl TIA for InMemoryTIA {
//    fn tick(&mut self) {
//        self.frame_cycle_counter += 1;

//        let offset = (self.frame_cycle_counter - 1) % cmn::CYCLES_PER_SCAN_LINE;
//        if self.wsync && offset == 0 {
//            self.rdy(false);
//        }
//        if offset < cmn::COL_DRAWABLE_AREA_START {
//            return;
//        }

//        let scan_line = (self.frame_cycle_counter - 1) / cmn::CYCLES_PER_SCAN_LINE;
//        if scan_line < cmn::ROW_VERTICAL_SYNC_END {
//            return;
//        }

//        if !bits::tst_bits(self.registers[cmn::regs::VBLANK], bits::BIT_D1) {
//            return;
//        }

//        self.tv
//            .borrow_mut()
//            .render(scan_line, offset, self.registers[cmn::regs::COLUBK]);
//    }

//    #[allow(non_snake_case)]
//    fn connect_rdy(&mut self, rdy: Rc<RefCell<dyn RDY>>) {
//        self.rdy = rdy.clone();
//    }

//    #[allow(non_snake_case)]
//    #[inline]
//    fn rdy(&mut self, state: bool) {
//        self.wsync = state;
//        self.rdy.borrow_mut().set_state(state);
//    }
//}

//#[allow(non_snake_case)]
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use crate::tests::*;

//    #[test]
//    fn render_empty_frame() {
//        let tv =
//            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::CYCLES_PER_SCAN_LINE }>::default();
//        let tv = Rc::new(RefCell::new(tv));
//        let mut tia = InMemoryTIA::new(tv.clone());

//        tia.tick_n(cmn::ntsc::CYCLES_PER_FRAME * 2);
//        assert_eq!(tia.frame_counter(), 0);

//        check_display!(tv.borrow().buffer(), 0x00);
//    }

//    #[test]
//    fn render_with_VBLANK_always_on() {
//        let tv =
//            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::CYCLES_PER_SCAN_LINE }>::default();
//        let tv = Rc::new(RefCell::new(tv));
//        let mut tia = InMemoryTIA::new(tv.clone());

//        let colubk = 0x1F;
//        tia.set(cmn::regs::VBLANK, bits::BIT_D1);
//        tia.set(cmn::regs::COLUBK, colubk);

//        tia.tick_n(cmn::ntsc::CYCLES_PER_FRAME);

//        check_display!(
//            tv.borrow().buffer(),
//            (
//                cmn::ntsc::ROW_VERTICAL_BLANK_START,
//                cmn::COL_DRAWABLE_AREA_START,
//                cmn::ntsc::ROW_OVERSCAN_END,
//                cmn::COL_DRAWABLE_AREA_END,
//            ),
//            (colubk, 0x00)
//        );
//    }

//    #[test]
//    fn render_solid_display() {
//        let tv =
//            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::CYCLES_PER_SCAN_LINE }>::default();
//        let tv = Rc::new(RefCell::new(tv));
//        let mut tia = InMemoryTIA::new(tv.clone());

//        tia.set(cmn::regs::VSYNC, bits::BIT_D1);
//        let colubk = 0xFF;
//        tia.set(cmn::regs::COLUBK, colubk);

//        tia.set(cmn::regs::VBLANK, 0x00);
//        tia.tick_n(cmn::ntsc::CYCLES_PER_VERTICAL_SYNC);
//        tia.set(cmn::regs::VSYNC, 0x00);
//        tia.tick_n(cmn::ntsc::CYCLES_PER_VERTICAL_BLANK);
//        tia.set(cmn::regs::VBLANK, bits::BIT_D1);
//        tia.tick_n(cmn::ntsc::CYCLES_PER_DRAWABLE_AREA_AND_HBLANK);
//        tia.set(cmn::regs::VBLANK, 0x00);
//        tia.tick_n(cmn::ntsc::CYCLES_PER_OVERSCAN);

//        assert_eq!(tia.frame_counter(), 1);
//        check_display!(
//            tv.borrow().buffer(),
//            (
//                cmn::ntsc::ROW_DRAWABLE_AREA_START,
//                cmn::COL_DRAWABLE_AREA_START,
//                cmn::ntsc::ROW_DRAWABLE_AREA_END,
//                cmn::COL_DRAWABLE_AREA_END
//            ),
//            (colubk, 0x00)
//        );
//    }

//    #[test]
//    fn with_VSYNC() {
//        let tv =
//            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::CYCLES_PER_SCAN_LINE }>::default();
//        let tv = Rc::new(RefCell::new(tv));
//        let mut tia = InMemoryTIA::new(tv.clone());

//        tia.set(cmn::regs::VBLANK, bits::BIT_D1);

//        tia.set(cmn::regs::COLUBK, 0x11);
//        tia.tick_n(cmn::ntsc::CYCLES_PER_VERTICAL_SYNC);
//        tia.tick_n(cmn::CYCLES_PER_SCAN_LINE);

//        tia.set(cmn::regs::VSYNC, bits::BIT_D1);
//        tia.set(cmn::regs::COLUBK, 0x22);
//        tia.tick_n(cmn::ntsc::CYCLES_PER_VERTICAL_SYNC);
//        tia.set(cmn::regs::VSYNC, 0x00);
//        tia.tick_n(cmn::CYCLES_PER_SCAN_LINE);
//        assert_eq!(tia.frame_counter(), 1);
//        check_display!(
//            tv.borrow().buffer(),
//            (
//                cmn::ntsc::ROW_VERTICAL_BLANK_START,
//                cmn::COL_DRAWABLE_AREA_START,
//                cmn::ntsc::ROW_VERTICAL_BLANK_START + 1,
//                cmn::COL_DRAWABLE_AREA_END
//            ),
//            (0x22, 0x00)
//        );
//    }

//    #[test]
//    fn with_WSYNC() {
//        let tv =
//            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::CYCLES_PER_SCAN_LINE }>::default();
//        let tv = Rc::new(RefCell::new(tv));
//        let rdy = Rc::new(RefCell::new(SpyRDY::new(false)));
//        let mut tia = InMemoryTIA::new(tv);
//        tia.connect_rdy(rdy.clone());

//        // 0th scan line.
//        tia.tick();
//        assert!(!rdy.borrow().state());
//        tia.set(cmn::regs::WSYNC, 0x00);
//        assert!(rdy.borrow().state());
//        tia.tick();
//        assert!(rdy.borrow().state());
//        tia.tick_n(cmn::CYCLES_PER_SCAN_LINE - 3);
//        assert!(rdy.borrow().state());
//        tia.tick();
//        assert!(rdy.borrow().state());
//        // 1st scan line.
//        tia.tick();
//        assert!(!rdy.borrow().state());
//        tia.tick_n(10 * cmn::CYCLES_PER_SCAN_LINE - 1);
//        assert!(!rdy.borrow().state());
//        // 11th scan line.
//        tia.tick();
//        assert!(!rdy.borrow().state());
//        tia.set(cmn::regs::WSYNC, 0x00);
//        assert!(rdy.borrow().state());
//        tia.tick();
//        assert!(rdy.borrow().state());
//        tia.tick_n(cmn::CYCLES_PER_SCAN_LINE - 3);
//        assert!(rdy.borrow().state());
//        tia.tick();
//        assert!(rdy.borrow().state());
//        tia.tick();
//        assert!(!rdy.borrow().state());
//    }

//    struct SpyRDY {
//        state: bool,
//    }

//    impl SpyRDY {
//        pub fn new(state: bool) -> Self {
//            Self { state }
//        }
//    }

//    impl RDY for SpyRDY {
//        fn state(&self) -> bool {
//            self.state
//        }

//        fn set_state(&mut self, state: bool) {
//            self.state = !self.state;
//            // NOTE: This ensures it is not called more than twice.
//            assert_eq!(state, self.state);
//        }
//    }
//}
