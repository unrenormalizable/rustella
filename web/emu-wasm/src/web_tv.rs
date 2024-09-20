use rustella::*;
use wasm_bindgen::prelude::*;

/// NOTE: Pretty much a copy of InMemoryTV, figure out a way to abstract it out.
pub struct NtscWebTV {
    config: tia::TVConfig<{ tia::NTSC_SCANLINES }, { tia::NTSC_PIXELS_PER_SCANLINE }>,
    /// Number of times VSYNC has been called.
    curr_scanline: usize,
    curr_pixel: usize,
    curr_scanline_buffer: [u32; tia::NTSC_PIXELS_PER_SCANLINE],
    /// JS callback.
    render_scanline_fn: JsValue,
}

impl NtscWebTV {
    pub fn new(render_scanline_fn: JsValue) -> Self {
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
            curr_scanline_buffer: [0x00; tia::NTSC_PIXELS_PER_SCANLINE],
            render_scanline_fn,
        }
    }

    fn render_pixel_core(&mut self, color: u8) {
        if self.curr_scanline < self.config.vsync_scanlines() {
            return;
        }

        if self.curr_pixel < self.config.hblank_pixels() {
            return;
        }

        self.curr_scanline_buffer[self.curr_pixel] = 1 + self.config.color_map()[color as usize];
    }

    fn send_scanline_to_js(&self) {
        let js_pixel_arr =
            js_sys::Uint32Array::new_with_length(self.curr_scanline_buffer.len() as u32);
        js_pixel_arr.copy_from(&self.curr_scanline_buffer);
        self.render_scanline_fn
            .clone()
            .unchecked_into::<js_sys::Function>()
            .call2(
                &JsValue::null(),
                &JsValue::from(self.curr_scanline),
                &js_pixel_arr,
            )
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
        let new_scanline =
            (self.curr_scanline + offset / tia::NTSC_PIXELS_PER_SCANLINE) % tia::NTSC_SCANLINES;
        if new_scanline != self.curr_scanline {
            self.send_scanline_to_js();
            self.curr_scanline = new_scanline;
        }
    }

    fn vsync(&mut self) {
        self.curr_scanline = 0;
        self.curr_pixel = 0;
    }
}
