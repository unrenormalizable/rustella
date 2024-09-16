use core::fmt::Debug;

#[derive(Debug)]
pub struct TVConfig {
    pub vsync_scanlines: usize,
    pub draw_scanlines: usize,
    pub hblank_pixels: usize,
    pub draw_pixels: usize,
}

pub trait TV: Debug {
    /// Render current pixel with the given color.
    fn render_pixel(&mut self, color: u8);

    /// Initiate VSYNC.
    fn vsync(&mut self);
}

#[derive(Debug)]
pub struct InMemoryTV<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> {
    /// Number of times VSYNC has been called.
    frame_counter: u64,
    scanline: usize,
    pixel: usize,
    config: TVConfig,
    buffer: [[u8; PIXELS_PER_SCANLINE]; SCANLINES],
    /// Total duration the for rendering all frames so far.
    duration: u64,
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize>
    InMemoryTV<SCANLINES, PIXELS_PER_SCANLINE>
{
    pub fn buffer(&self) -> [[u8; PIXELS_PER_SCANLINE]; SCANLINES] {
        self.buffer
    }

    pub fn frame_counter(&self) -> u64 {
        self.frame_counter
    }

    pub fn duration(&self) -> u64 {
        self.duration
    }
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize>
    InMemoryTV<SCANLINES, PIXELS_PER_SCANLINE>
{
    pub fn new(config: TVConfig) -> Self {
        Self::new_testable(0, 0, config)
    }

    fn new_testable(scanline: usize, offset: usize, config: TVConfig) -> Self {
        if PIXELS_PER_SCANLINE != (config.hblank_pixels + config.draw_pixels) {
            panic!("Config error. Pixels mismatch.")
        }

        if SCANLINES != (config.vsync_scanlines + config.draw_scanlines) {
            panic!("Config error. Scanlines mismatch.")
        }

        Self {
            frame_counter: 0,
            scanline,
            pixel: offset,
            config,
            buffer: [[0x00; PIXELS_PER_SCANLINE]; SCANLINES],
            duration: 0,
        }
    }

    fn render_pixel_core(&mut self, color: u8) {
        if self.scanline < self.config.vsync_scanlines {
            return;
        }

        if self.pixel < self.config.hblank_pixels {
            return;
        }

        self.buffer[self.scanline][self.pixel] = color;
    }
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> TV
    for InMemoryTV<PIXELS_PER_SCANLINE, SCANLINES>
{
    fn render_pixel(&mut self, color: u8) {
        self.render_pixel_core(color);

        let offset = self.pixel + 1;
        self.pixel = offset % PIXELS_PER_SCANLINE;
        self.scanline += offset / PIXELS_PER_SCANLINE;
        if self.scanline >= SCANLINES {
            self.scanline = SCANLINES - 1;
        }
    }

    fn vsync(&mut self) {
        self.scanline = 0;
        self.pixel = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use test_case::test_case;

    const PIXELS_PER_SCAN_LINE: usize = 3;
    const SCANLINES: usize = 3;

    const CONFIG: TVConfig = TVConfig {
        hblank_pixels: 1,
        draw_pixels: 2,
        vsync_scanlines: 1,
        draw_scanlines: 2,
    };

    type TestableTV = InMemoryTV<SCANLINES, PIXELS_PER_SCAN_LINE>;

    #[test_case(0, 0)]
    #[test_case(0, 1)]
    #[test_case(0, 2)]
    #[test_case(1, 0)]
    #[test_case(2, 0)]
    fn render_pixel_none_on_non_draw_areas(scanline: usize, pixel: usize) {
        let mut tv = TestableTV::new_testable(scanline, pixel, CONFIG);

        tv.render_pixel(0x01);

        check_2D_array!(tv.buffer(), 0x00);
    }

    #[test_case(1, 1)]
    #[test_case(1, 2)]
    #[test_case(2, 1)]
    #[test_case(2, 2)]
    fn render_pixel_on_draw_areas(scanline: usize, pixel: usize) {
        let mut tv = TestableTV::new_testable(scanline, pixel, CONFIG);

        tv.render_pixel(0x01);

        let mut buffer = tv.buffer();
        assert_eq!(buffer[scanline][pixel], 0x01);
        buffer[scanline][pixel] = 0x00;
        check_2D_array!(buffer, 0x00);
    }

    #[test]
    fn increment_pixel() {
        let mut tv = TestableTV::new_testable(1, 1, CONFIG);
        tv.render_pixel(0x01);
        tv.render_pixel(0x02);

        assert_eq!(tv.buffer()[1], [0x00, 0x01, 0x02]);
    }

    #[test]
    fn increment_scanline() {
        let mut tv = TestableTV::new_testable(1, 2, CONFIG);
        tv.render_pixel(0x01);
        tv.render_pixel(0x02);
        tv.render_pixel(0x03);

        assert_eq!(tv.buffer()[1], [0x00, 0x00, 0x01]);
        assert_eq!(tv.buffer()[2], [0x00, 0x03, 0x00]);
    }

    #[test]
    fn increment_scanline_past_end() {
        let mut tv = TestableTV::new_testable(2, 2, CONFIG);

        tv.render_pixel(0x01);
        assert_eq!(tv.buffer()[2], [0x00, 0x00, 0x01]);

        tv.render_pixel(0x02);
        assert_eq!(tv.buffer()[2], [0x00, 0x00, 0x01]);

        tv.render_pixel(0x03);
        assert_eq!(tv.buffer()[2], [0x00, 0x03, 0x01]);
    }

    #[test]
    fn vsync_resets_sanline() {
        let mut tv = TestableTV::new_testable(1, 1, CONFIG);

        tv.vsync();
        tv.render_pixel(0x01);
        tv.render_pixel(0x02);
        tv.render_pixel(0x03);
        tv.render_pixel(0x04);
        check_2D_array!(tv.buffer(), 0x00);
        tv.render_pixel(0x04);
        assert_eq!(tv.buffer()[1], [0x00, 0x04, 0x00]);
    }
}
