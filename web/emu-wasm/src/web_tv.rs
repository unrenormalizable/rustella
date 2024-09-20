use rustella::*;
use wasm_bindgen::prelude::*;

/// NOTE: Pretty much a copy of InMemoryTV, figure out a way to abstract it out.
pub struct NtscWebTV {
    config: tia::TVConfig<{ tia::NTSC_SCANLINES }, { tia::NTSC_PIXELS_PER_SCANLINE }>,
    /// Number of times VSYNC has been called.
    curr_scanline: usize,
    curr_pixel: usize,
    video_buffer: [u8; tia::NTSC_PIXELS_PER_SCANLINE * tia::NTSC_SCANLINES],
    /// JS callback.
    render_frame_fn: JsValue,
}

impl NtscWebTV {
    pub fn new(render_frame_fn: JsValue) -> Self {
        let config = tia::ntsc_tv_config();

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
            curr_scanline: 0,
            curr_pixel: 0,
            config,
            video_buffer: [0x00; tia::NTSC_PIXELS_PER_SCANLINE * tia::NTSC_SCANLINES],
            render_frame_fn,
        }
    }

    fn render_pixel_core(&mut self, color: u8) {
        if self.curr_scanline < self.config.vsync_scanlines() {
            return;
        }

        if self.curr_pixel < self.config.hblank_pixels() {
            return;
        }

        self.video_buffer[tia::NTSC_PIXELS_PER_SCANLINE * self.curr_scanline + self.curr_pixel] =
            color;
    }

    fn send_scanline_to_js(&self) {
        let js_pixel_arr = js_sys::Uint8Array::new_with_length(self.video_buffer.len() as u32);
        js_pixel_arr.copy_from(&self.video_buffer);
        self.render_frame_fn
            .clone()
            .unchecked_into::<js_sys::Function>()
            .call1(&JsValue::null(), &js_pixel_arr)
            .unwrap();
    }
}

impl tia::TV<{ tia::NTSC_SCANLINES }, { tia::NTSC_PIXELS_PER_SCANLINE }> for NtscWebTV {
    fn config(&self) -> &tia::TVConfig<{ tia::NTSC_SCANLINES }, { tia::NTSC_PIXELS_PER_SCANLINE }> {
        &self.config
    }

    fn render_pixel(&mut self, color: u8) {
        self.render_pixel_core(color);

        let offset = self.curr_pixel + 1;
        self.curr_pixel = offset % tia::NTSC_PIXELS_PER_SCANLINE;
        self.curr_scanline =
            (self.curr_scanline + offset / tia::NTSC_PIXELS_PER_SCANLINE) % tia::NTSC_SCANLINES;
    }

    fn vsync(&mut self) {
        self.curr_scanline = 0;
        self.curr_pixel = 0;

        self.send_scanline_to_js();
    }
}
