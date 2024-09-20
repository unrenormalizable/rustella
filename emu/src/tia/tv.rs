use core::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct TVConfig<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> {
    // Rows.
    scanlines: usize,
    vsync_scanlines: usize,
    vblank_scanlines: usize,
    draw_scanlines: usize,
    overscan_scanlines: usize,
    // Columns.
    pixels_per_scanline: usize,
    hblank_pixels: usize,
    draw_pixels: usize,
    // colors
    color_map: [u32; 256],
}

pub trait TV<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> {
    fn config(&self) -> &TVConfig<SCANLINES, PIXELS_PER_SCANLINE>;

    /// Render current pixel with the given color.
    fn render_pixel(&mut self, color: u8);

    /// Initiate VSYNC.
    fn vsync(&mut self);
}

#[derive(Debug)]
pub struct InMemoryTV<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> {
    /// Number of times VSYNC has been called.
    frame_counter: u64,
    curr_scanline: usize,
    curr_pixel: usize,
    config: TVConfig<SCANLINES, PIXELS_PER_SCANLINE>,
    buffer: [[u8; PIXELS_PER_SCANLINE]; SCANLINES],
    /// Total duration the for rendering all frames so far.
    duration: u64,
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize>
    InMemoryTV<SCANLINES, PIXELS_PER_SCANLINE>
{
    pub fn new(config: TVConfig<SCANLINES, PIXELS_PER_SCANLINE>) -> Self {
        Self::new_testable(0, 0, config)
    }

    pub fn new_testable(
        scanline: usize,
        pixel: usize,
        config: TVConfig<SCANLINES, PIXELS_PER_SCANLINE>,
    ) -> Self {
        if config.pixels_per_scanline() != (config.hblank_pixels() + config.draw_pixels()) {
            panic!("Config error. Pixels mismatch.")
        }

        if config.scanlines()
            != (config.vsync_scanlines()
                + config.vblank_scanlines()
                + config.draw_scanlines()
                + config.overscan_scanlines())
        {
            panic!("Config error. Scanlines mismatch.")
        }

        Self {
            frame_counter: 0,
            curr_scanline: scanline,
            curr_pixel: pixel,
            config,
            buffer: [[0x00; PIXELS_PER_SCANLINE]; SCANLINES],
            duration: 0,
        }
    }

    pub fn buffer(&self) -> [[u8; PIXELS_PER_SCANLINE]; SCANLINES] {
        self.buffer
    }

    pub fn frame_counter(&self) -> u64 {
        self.frame_counter
    }

    pub fn duration(&self) -> u64 {
        self.duration
    }

    fn render_pixel_core(&mut self, color: u8) {
        if self.curr_scanline < self.config.vsync_scanlines() {
            return;
        }

        if self.curr_pixel < self.config.hblank_pixels() {
            return;
        }

        self.buffer[self.curr_scanline][self.curr_pixel] = color;
    }
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> TV<SCANLINES, PIXELS_PER_SCANLINE>
    for InMemoryTV<SCANLINES, PIXELS_PER_SCANLINE>
{
    fn config(&self) -> &TVConfig<SCANLINES, PIXELS_PER_SCANLINE> {
        &self.config
    }

    fn render_pixel(&mut self, color: u8) {
        self.render_pixel_core(color);

        let offset = self.curr_pixel + 1;
        self.curr_pixel = offset % PIXELS_PER_SCANLINE;
        self.curr_scanline = (self.curr_scanline + offset / PIXELS_PER_SCANLINE) % SCANLINES;
    }

    fn vsync(&mut self) {
        self.curr_scanline = 0;
        self.curr_pixel = 0;
    }
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> Default
    for TVConfig<SCANLINES, PIXELS_PER_SCANLINE>
{
    fn default() -> Self {
        Self {
            scanlines: SCANLINES,
            vsync_scanlines: 1,
            vblank_scanlines: 0,
            draw_scanlines: SCANLINES - 1,
            overscan_scanlines: 0,
            pixels_per_scanline: PIXELS_PER_SCANLINE,
            hblank_pixels: 1,
            draw_pixels: PIXELS_PER_SCANLINE - 1,
            color_map: [0x00u32; 256],
        }
    }
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize>
    TVConfig<SCANLINES, PIXELS_PER_SCANLINE>
{
    pub fn new(
        vsync_scanlines: usize,
        vblank_scanlines: usize,
        draw_scanlines: usize,
        hblank_pixels: usize,
        color_map: [u32; 256],
    ) -> Self {
        Self {
            scanlines: SCANLINES,
            vsync_scanlines,
            vblank_scanlines,
            draw_scanlines,
            overscan_scanlines: SCANLINES - vsync_scanlines - vblank_scanlines - draw_scanlines,
            pixels_per_scanline: PIXELS_PER_SCANLINE,
            hblank_pixels,
            draw_pixels: PIXELS_PER_SCANLINE - hblank_pixels,
            color_map,
        }
    }

    #[inline]
    pub fn color_map(&self) -> &[u32; 256] {
        &self.color_map
    }

    #[inline]
    pub fn scanlines(&self) -> usize {
        self.scanlines
    }

    #[inline]
    pub fn vsync_scanlines(&self) -> usize {
        self.vsync_scanlines
    }

    #[inline]
    pub fn vblank_scanlines(&self) -> usize {
        self.vblank_scanlines
    }

    #[inline]
    pub fn draw_scanlines(&self) -> usize {
        self.draw_scanlines
    }

    #[inline]
    pub fn overscan_scanlines(&self) -> usize {
        self.overscan_scanlines
    }

    #[inline]
    pub fn pixels_per_scanline(&self) -> usize {
        self.pixels_per_scanline
    }

    #[inline]
    pub fn hblank_pixels(&self) -> usize {
        self.hblank_pixels
    }

    #[inline]
    pub fn draw_pixels(&self) -> usize {
        self.draw_pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
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

    #[test_case(0, 0)]
    #[test_case(0, 1)]
    #[test_case(0, 2)]
    #[test_case(1, 0)]
    #[test_case(2, 0)]
    fn render_pixel_none_on_non_draw_areas(scanline: usize, pixel: usize) {
        let mut tv = TestableTV::new_testable(scanline, pixel, TestableTVConfig::default());

        tv.render_pixel(0x01);

        check_2D_array!(tv.buffer(), 0x00);
    }

    #[test_case(1, 1)]
    #[test_case(1, 2)]
    #[test_case(2, 1)]
    #[test_case(2, 2)]
    fn render_pixel_on_draw_areas(scanline: usize, pixel: usize) {
        let mut tv = TestableTV::new_testable(scanline, pixel, TestableTVConfig::default());

        tv.render_pixel(0x01);

        let mut buffer = tv.buffer();
        assert_eq!(buffer[scanline][pixel], 0x01);
        buffer[scanline][pixel] = 0x00;
        check_2D_array!(buffer, 0x00);
    }

    #[test]
    fn increment_pixel() {
        let mut tv = TestableTV::new_testable(1, 1, TestableTVConfig::default());
        tv.render_pixel(0x01);
        tv.render_pixel(0x02);

        assert_eq!(tv.buffer()[1], [0x00, 0x01, 0x02]);
    }

    #[test]
    fn pixel_wraparound_increment_scanline() {
        let mut tv = TestableTV::new_testable(1, 2, TestableTVConfig::default());
        tv.render_pixel(0x01);
        tv.render_pixel(0x02);
        tv.render_pixel(0x03);

        assert_eq!(tv.buffer()[1], [0x00, 0x00, 0x01]);
        assert_eq!(tv.buffer()[2], [0x00, 0x03, 0x00]);
    }

    #[test]
    fn scanline_wraparound() {
        let mut tv = TestableTV::new_testable(2, 2, TestableTVConfig::default());

        tv.render_pixel(0x01);
        assert_eq!(tv.buffer()[2], [0x00, 0x00, 0x01]);

        (0..6u8).for_each(|n| tv.render_pixel(n));
        assert_eq!(tv.buffer()[0], [0x00, 0x00, 0x00]);
        assert_eq!(tv.buffer()[1], [0x00, 0x04, 0x05]);
    }

    #[test]
    fn vsync_resets_scanline() {
        let mut tv = TestableTV::new_testable(1, 1, TestableTVConfig::default());

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
