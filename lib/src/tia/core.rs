use super::{cmn, tv};

const COUNT_REGISTERS: usize = cmn::Register::CXCLR as usize;
const BIT_D1: u8 = 0x01 << 1;

/// Refer:
/// - module README.md
/// - https://www.atarihq.com/danb/files/TIA_HW_Notes.txt
#[derive(Debug)]
pub struct TIA<'a> {
    counter: usize,
    registers: [u8; COUNT_REGISTERS],
    tv: &'a mut dyn tv::TV,
}

impl<'a> TIA<'a> {
    pub fn new(tv: &'a mut dyn tv::TV) -> Self {
        Self {
            registers: [0x00; COUNT_REGISTERS],
            counter: 0,
            tv,
        }
    }

    pub fn set_register(&mut self, reg: cmn::Register, val: u8) {
        #[cfg(debug_assertions)]
        self.check_unsupported_register_flags(&reg, val);

        self.registers[reg as usize] = val;
    }

    pub fn tick(&mut self) {
        self.counter += 1;

        let scan_line = (self.counter - 1) / cmn::PIXELS_PER_SCAN_LINE;
        if scan_line < cmn::ROW_VERTICAL_SYNC_END {
            return;
        }

        let offset = (self.counter - 1) % cmn::PIXELS_PER_SCAN_LINE;
        if offset < cmn::COL_DRAWABLE_AREA_START {
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
            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::PIXELS_PER_SCAN_LINE }>::default();
        let mut tia = TIA::new(&mut tv);

        (0..(cmn::ntsc::CYCLES_PER_FRAME * 2)).for_each(|_| tia.tick());

        check_display!(tv.buffer(), 0x00);
    }

    #[test]
    fn render_with_vblank_always_on() {
        let mut tv =
            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::PIXELS_PER_SCAN_LINE }>::default();
        let mut tia = TIA::new(&mut tv);

        let colubk = 0x1F;
        tia.set_register(cmn::Register::VBLANK, BIT_D1);
        tia.set_register(cmn::Register::COLUBK, colubk);

        (0..cmn::ntsc::CYCLES_PER_FRAME).for_each(|_| tia.tick());

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
            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::PIXELS_PER_SCAN_LINE }>::default();
        let mut tia = TIA::new(&mut tv);

        let colubk = 0xFF;
        tia.set_register(cmn::Register::COLUBK, colubk);

        tia.set_register(cmn::Register::VBLANK, 0x00);
        (0..cmn::ntsc::CYCLES_PER_VERTICAL_SYNC).for_each(|_| tia.tick());
        (0..cmn::ntsc::CYCLES_PER_VERTICAL_BLANK).for_each(|_| tia.tick());
        tia.set_register(cmn::Register::VBLANK, BIT_D1);
        (0..(cmn::ntsc::CYCLES_PER_FRAME
            - cmn::ntsc::CYCLES_PER_VERTICAL_SYNC
            - cmn::ntsc::CYCLES_PER_VERTICAL_BLANK
            - cmn::ntsc::CYCLES_PER_OVERSCAN))
            .for_each(|_| tia.tick());
        tia.set_register(cmn::Register::VBLANK, 0x00);
        (0..(cmn::ntsc::CYCLES_PER_OVERSCAN)).for_each(|_| tia.tick());

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
            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::PIXELS_PER_SCAN_LINE }>::default();
        let mut tia = TIA::new(&mut tv);

        let colubk = 0xEE;
        tia.set_register(cmn::Register::COLUBK, colubk);

        tia.set_register(cmn::Register::VBLANK, 0x00);
        (0..cmn::ntsc::CYCLES_PER_VERTICAL_SYNC).for_each(|_| tia.tick());
        (0..cmn::ntsc::CYCLES_PER_VERTICAL_BLANK).for_each(|_| tia.tick());
        tia.set_register(cmn::Register::VBLANK, BIT_D1);
        (0..(cmn::ntsc::CYCLES_PER_FRAME
            - cmn::ntsc::CYCLES_PER_VERTICAL_SYNC
            - cmn::ntsc::CYCLES_PER_VERTICAL_BLANK
            - cmn::ntsc::CYCLES_PER_OVERSCAN))
            .for_each(|_| tia.tick());
        tia.set_register(cmn::Register::VBLANK, 0x00);
        (0..(cmn::ntsc::CYCLES_PER_OVERSCAN)).for_each(|_| tia.tick());

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
            tv::InMemoryTV::<{ cmn::ntsc::SCAN_LINES }, { cmn::PIXELS_PER_SCAN_LINE }>::default();
        let mut tia = TIA::new(&mut tv);

        let colubk = 0xEE;
        tia.set_register(cmn::Register::COLUBK, colubk);

        tia.set_register(cmn::Register::VBLANK, 0x00);
        (0..cmn::ntsc::CYCLES_PER_VERTICAL_SYNC).for_each(|_| tia.tick());
        (0..cmn::ntsc::CYCLES_PER_VERTICAL_BLANK).for_each(|_| tia.tick());
        tia.set_register(cmn::Register::VBLANK, BIT_D1);
        (0..(cmn::ntsc::CYCLES_PER_FRAME
            - cmn::ntsc::CYCLES_PER_VERTICAL_SYNC
            - cmn::ntsc::CYCLES_PER_VERTICAL_BLANK
            - cmn::ntsc::CYCLES_PER_OVERSCAN))
            .for_each(|_| tia.tick());
        tia.set_register(cmn::Register::VBLANK, 0x00);
        (0..(cmn::ntsc::CYCLES_PER_OVERSCAN)).for_each(|_| tia.tick());

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
}
