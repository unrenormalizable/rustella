use super::{cmn, tv};

const COUNT_REGISTERS: usize = cmn::Register::CXCLR as usize;
const BIT_D1: u8 = 0x01 << 1;

/// Refer:
/// - module README.md
/// - https://www.atarihq.com/danb/files/TIA_HW_Notes.txt
pub struct TIA<'a> {
    counter: usize,
    registers: [u8; COUNT_REGISTERS],
    wsync: bool,
    rdy: &'a mut dyn FnMut(bool),
    tv: &'a mut dyn tv::TV,
}

impl<'a> TIA<'a> {
    pub fn new(tv: &'a mut dyn tv::TV, rdy: &'a mut dyn FnMut(bool)) -> Self {
        Self {
            counter: 0,
            registers: [0x00; COUNT_REGISTERS],
            wsync: false,
            rdy,
            tv,
        }
    }

    pub fn set_register(&mut self, reg: cmn::Register, val: u8) {
        #[cfg(debug_assertions)]
        self.check_unsupported_register_flags(&reg, val);

        if let cmn::Register::WSYNC = reg {
            self.set_RDY(true);
        }

        self.registers[reg as usize] = val;
    }

    #[inline]
    pub fn tick_n(&mut self, n: usize) {
        (0..n).for_each(|_| self.tick())
    }

    pub fn tick(&mut self) {
        self.counter += 1;

        let offset = (self.counter - 1) % cmn::CYCLES_PER_SCAN_LINE;
        if self.wsync && offset == 0 {
            self.set_RDY(false);
        }
        if offset < cmn::COL_DRAWABLE_AREA_START {
            return;
        }

        let scan_line = (self.counter - 1) / cmn::CYCLES_PER_SCAN_LINE;
        if scan_line < cmn::ROW_VERTICAL_SYNC_END {
            return;
        }

        if !crate::bits::tst_bits(self.registers[cmn::Register::VBLANK as usize], BIT_D1) {
            return;
        }

        self.tv.render(
            scan_line,
            offset,
            self.registers[cmn::Register::COLUBK as usize],
        );
    }

    #[allow(non_snake_case)]
    fn set_RDY(&mut self, on: bool) {
        self.wsync = on;
        (self.rdy)(on);
    }

    #[cfg(debug_assertions)]
    fn check_unsupported_register_flags(&self, reg: &cmn::Register, val: u8) {
        if let cmn::Register::VBLANK = reg {
            let x = val & !BIT_D1;
            assert!(x == 0, "Unsupported {reg:?} <= 0x{val:02X}.")
        }
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn render_empty_frame() {
        let mut tv =
            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::CYCLES_PER_SCAN_LINE }>::default();
        let rdy = &mut nop_rdy;
        let mut tia = TIA::new(&mut tv, rdy);

        tia.tick_n(cmn::ntsc::CYCLES_PER_FRAME * 2);

        check_display!(tv.buffer(), 0x00);
    }

    #[test]
    fn render_with_vblank_always_on() {
        let mut tv =
            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::CYCLES_PER_SCAN_LINE }>::default();
        let rdy = &mut nop_rdy;
        let mut tia = TIA::new(&mut tv, rdy);

        let colubk = 0x1F;
        tia.set_register(cmn::Register::VBLANK, BIT_D1);
        tia.set_register(cmn::Register::COLUBK, colubk);

        tia.tick_n(cmn::ntsc::CYCLES_PER_FRAME);

        check_display!(
            tv.buffer(),
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
        let mut tv =
            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::CYCLES_PER_SCAN_LINE }>::default();
        let rdy = &mut nop_rdy;
        let mut tia = TIA::new(&mut tv, rdy);

        let colubk = 0xFF;
        tia.set_register(cmn::Register::COLUBK, colubk);

        tia.set_register(cmn::Register::VBLANK, 0x00);
        tia.tick_n(cmn::ntsc::CYCLES_PER_VERTICAL_SYNC);
        tia.tick_n(cmn::ntsc::CYCLES_PER_VERTICAL_BLANK);
        tia.set_register(cmn::Register::VBLANK, BIT_D1);
        tia.tick_n(cmn::ntsc::CYCLES_PER_DRAWABLE_AREA_AND_HBLANK);
        tia.set_register(cmn::Register::VBLANK, 0x00);
        tia.tick_n(cmn::ntsc::CYCLES_PER_OVERSCAN);

        check_display!(
            tv.buffer(),
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
        let mut tv =
            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::CYCLES_PER_SCAN_LINE }>::default();
        let rdy = &mut nop_rdy;
        let mut tia = TIA::new(&mut tv, rdy);

        let colubk = 0xEE;
        tia.set_register(cmn::Register::COLUBK, colubk);

        tia.set_register(cmn::Register::VBLANK, 0x00);
        tia.tick_n(cmn::ntsc::CYCLES_PER_VERTICAL_SYNC);
        tia.tick_n(cmn::ntsc::CYCLES_PER_VERTICAL_BLANK);
        tia.set_register(cmn::Register::VBLANK, BIT_D1);
        tia.tick_n(cmn::ntsc::CYCLES_PER_DRAWABLE_AREA_AND_HBLANK);
        tia.set_register(cmn::Register::VBLANK, 0x00);
        tia.tick_n(cmn::ntsc::CYCLES_PER_OVERSCAN);

        check_display!(
            tv.buffer(),
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
    fn with_WSYNC() {
        let mut tv =
            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::CYCLES_PER_SCAN_LINE }>::default();
        let waiting_on_wsync = core::cell::Cell::new(false);
        let mut rdy = |on: bool| {
            waiting_on_wsync.set(!waiting_on_wsync.get());
            // NOTE: This ensures it is not called more than twice.
            assert_eq!(on, waiting_on_wsync.get())
        };
        let mut tia = TIA::new(&mut tv, &mut rdy);

        // 0th scan line.
        tia.tick();
        assert!(!waiting_on_wsync.get());
        tia.set_register(cmn::Register::WSYNC, 0x00);
        assert!(waiting_on_wsync.get());
        tia.tick();
        assert!(waiting_on_wsync.get());
        tia.tick_n(cmn::CYCLES_PER_SCAN_LINE - 3);
        assert!(waiting_on_wsync.get());
        tia.tick();
        assert!(waiting_on_wsync.get());
        // 1st scan line.
        tia.tick();
        assert!(!waiting_on_wsync.get());
        tia.tick_n(10 * cmn::CYCLES_PER_SCAN_LINE - 1);
        assert!(!waiting_on_wsync.get());
        // 11th scan line.
        tia.tick();
        assert!(!waiting_on_wsync.get());
        tia.set_register(cmn::Register::WSYNC, 0x00);
        assert!(waiting_on_wsync.get());
        tia.tick();
        assert!(waiting_on_wsync.get());
        tia.tick_n(cmn::CYCLES_PER_SCAN_LINE - 3);
        assert!(waiting_on_wsync.get());
        tia.tick();
        assert!(waiting_on_wsync.get());
        tia.tick();
        assert!(!waiting_on_wsync.get());
    }

    fn nop_rdy(_: bool) {}
}
