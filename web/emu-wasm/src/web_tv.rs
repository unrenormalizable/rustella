use rustella::*;
use wasm_bindgen::prelude::*;

/// NOTE: Pretty much a copy of InMemoryTV, figure out a way to abstract it out.
pub struct NtscWebTV {
    config: tia::TVConfig<{ tia::NTSC_SCANLINES }, { tia::NTSC_PIXELS_PER_SCANLINE }>,
    /// Number of times VSYNC has been called.
    curr_scanline: usize,
    curr_pixel: usize,
    vsync_on: bool,
    video_buffer: [u8; tia::NTSC_PIXELS_PER_SCANLINE * tia::NTSC_SCANLINES],
    /// JS callback.
    render_frame_fn: JsValue,
}

impl NtscWebTV {
    pub fn new(render_frame_fn: JsValue) -> Self {
        let config = tia::ntsc_tv_config();

        Self {
            curr_scanline: 0,
            curr_pixel: 0,
            config,
            vsync_on: false,
            video_buffer: [0x00; tia::NTSC_PIXELS_PER_SCANLINE * tia::NTSC_SCANLINES],
            render_frame_fn,
        }
    }
}

impl tia::TV<{ tia::NTSC_SCANLINES }, { tia::NTSC_PIXELS_PER_SCANLINE }> for NtscWebTV {
    fn config(&self) -> &tia::TVConfig<{ tia::NTSC_SCANLINES }, { tia::NTSC_PIXELS_PER_SCANLINE }> {
        &self.config
    }

    fn post_vsync(&self) {
        let js_pixel_arr = js_sys::Uint8Array::new_with_length(self.video_buffer.len() as u32);
        js_pixel_arr.copy_from(&self.video_buffer);
        self.render_frame_fn
            .clone()
            .unchecked_into::<js_sys::Function>()
            .call1(&JsValue::null(), &js_pixel_arr)
            .unwrap();
    }

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
        self.video_buffer
            [tia::NTSC_PIXELS_PER_SCANLINE * self.current_scanline() + self.current_pixel()] =
            color;
    }

    #[inline]
    fn frame_counter(&self) -> u64 {
        0
    }

    #[inline]
    fn set_frame_counter(&mut self, _frames: u64) {}

    #[inline]
    fn vsync_on(&self) -> bool {
        self.vsync_on
    }

    #[inline]
    fn set_vsync_on(&mut self, on: bool) {
        self.vsync_on = on;
    }
}
