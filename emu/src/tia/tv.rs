use core::fmt::Debug;

pub trait TV<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> {
    fn config(&self) -> &TVConfig<SCANLINES, PIXELS_PER_SCANLINE>;

    /// Render current pixel with the given color.
    fn render_pixel(&mut self, color: u8) {
        self.render_pixel_core(color);

        let offset = self.current_pixel() + 1;
        self.set_current_pixel(offset % self.config().pixels_per_scanline());
        self.set_current_scanline(
            self.current_scanline() + offset / self.config().pixels_per_scanline(),
        );
    }

    #[inline]
    fn render_pixel_core(&mut self, color: u8) {
        if self.vsync_on() || self.current_scanline() >= self.config().scanlines() {
            return;
        }

        self.write_buffer(color);
    }

    #[inline]
    fn vsync_start(&mut self) {
        if !self.vsync_on() {
            self.set_vsync_on(true);
            self.set_current_scanline(0);
            self.set_frame_counter(self.frame_counter() + 1);
        }
    }

    #[inline]
    fn vsync_end(&mut self) {
        self.set_vsync_on(false);

        self.post_vsync();
    }

    fn current_scanline(&self) -> usize;

    fn set_current_scanline(&mut self, scanline: usize);

    fn current_pixel(&self) -> usize;

    fn set_current_pixel(&mut self, pixel: usize);

    fn write_buffer(&mut self, color: u8);

    fn post_vsync(&self);

    fn frame_counter(&self) -> u64;

    fn set_frame_counter(&mut self, frames: u64);

    fn vsync_on(&self) -> bool;

    fn set_vsync_on(&mut self, on: bool);
}

#[derive(Debug)]
pub struct InMemoryTV<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> {
    /// Number of times VSYNC has been called.
    frame_counter: u64,
    vsync_on: bool,
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
        Self {
            frame_counter: 0,
            vsync_on: false,
            curr_scanline: scanline,
            curr_pixel: pixel,
            config,
            buffer: [[0x00; PIXELS_PER_SCANLINE]; SCANLINES],
            duration: 0,
        }
    }

    #[inline]
    pub fn buffer(&self) -> [[u8; PIXELS_PER_SCANLINE]; SCANLINES] {
        self.buffer
    }

    #[inline]
    pub fn duration(&self) -> u64 {
        self.duration
    }
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> TV<SCANLINES, PIXELS_PER_SCANLINE>
    for InMemoryTV<SCANLINES, PIXELS_PER_SCANLINE>
{
    #[inline]
    fn config(&self) -> &TVConfig<SCANLINES, PIXELS_PER_SCANLINE> {
        &self.config
    }

    #[inline]
    fn post_vsync(&self) {}

    #[inline]
    fn current_scanline(&self) -> usize {
        self.curr_scanline
    }

    #[inline]
    fn set_current_scanline(&mut self, scanline: usize) {
        self.curr_scanline = scanline
    }

    #[inline]
    fn current_pixel(&self) -> usize {
        self.curr_pixel
    }

    #[inline]
    fn set_current_pixel(&mut self, pixel: usize) {
        self.curr_pixel = pixel
    }

    #[inline]
    fn write_buffer(&mut self, color: u8) {
        self.buffer[self.current_scanline()][self.current_pixel()] = color;
    }

    #[inline]
    fn frame_counter(&self) -> u64 {
        self.frame_counter
    }

    #[inline]
    fn set_frame_counter(&mut self, frames: u64) {
        self.frame_counter = frames
    }

    #[inline]
    fn vsync_on(&self) -> bool {
        self.vsync_on
    }

    #[inline]
    fn set_vsync_on(&mut self, on: bool) {
        self.vsync_on = on;
    }
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> Default
    for TVConfig<SCANLINES, PIXELS_PER_SCANLINE>
{
    fn default() -> Self {
        TVConfig::<SCANLINES, PIXELS_PER_SCANLINE>::new(PIXELS_PER_SCANLINE - 1, [0x00u32; 128])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TVConfig<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize> {
    // Rows.
    scanlines: usize,
    // Columns.
    pixels_per_scanline: usize,
    hblank_pixels: usize,
    visible_pixels: usize,
    // colors
    color_map: [u32; 128],
}

impl<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize>
    TVConfig<SCANLINES, PIXELS_PER_SCANLINE>
{
    pub fn new(draw_pixels: usize, color_map: [u32; 128]) -> Self {
        let ret = Self {
            scanlines: SCANLINES,
            pixels_per_scanline: PIXELS_PER_SCANLINE,
            hblank_pixels: PIXELS_PER_SCANLINE - draw_pixels,
            visible_pixels: draw_pixels,
            color_map,
        };

        if ret.pixels_per_scanline() != (ret.hblank_pixels() + ret.visible_pixels()) {
            panic!("Config error. Pixels mismatch.")
        }

        ret
    }

    #[inline]
    pub fn color_map(&self) -> &[u32; 128] {
        &self.color_map
    }

    #[inline]
    pub fn scanlines(&self) -> usize {
        self.scanlines
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
    pub fn visible_pixels(&self) -> usize {
        self.visible_pixels
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
    fn vsync_resets_scanline() {
        let mut tv = TestableTV::new_testable(2, 1, TestableTVConfig::default());

        tv.vsync_start();
        [0x01, 0x02, 0x03].iter().for_each(|&x| tv.render_pixel(x));
        tv.vsync_end();
        check_2D_array!(tv.buffer(), 0x00);
        tv.render_pixel(0x04);
        assert_eq!(tv.buffer()[0], [0x00, 0x00, 0x00]);
        assert_eq!(tv.buffer()[1], [0x00, 0x04, 0x00]);
        assert_eq!(tv.frame_counter(), 1);
    }
}
