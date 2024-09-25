mod web_tv;

use rustella::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

thread_local! {
    static ATARI: RefCell<NtscAtari> = panic!("initialized not called.");
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
fn initialize() {
    set_panic_hook();
    console_log!("Initialized emu_wasm...");
}

#[allow(non_snake_case)]
#[wasm_bindgen(js_name = ntscColorMap)]
pub fn ntsc_color_map() -> js_sys::Uint32Array {
    let cfg = tia::ntsc_tv_config();
    let map = cfg.color_map();
    let js_map = js_sys::Uint32Array::new_with_length(map.len() as u32);
    js_map.copy_from(map);
    js_map
}

#[derive(Default)]
#[wasm_bindgen]
pub struct Atari {}

#[allow(non_snake_case)]
#[wasm_bindgen]
impl Atari {
    #[wasm_bindgen(constructor)]
    pub fn new(render_scanline_fn: JsValue) -> Self {
        let tv = web_tv::NtscWebTV::new(render_scanline_fn);
        let atari = NtscAtari::new(Rc::new(RefCell::new(tv)));

        ATARI.set(atari);
        console_log!("Created new atari...");

        Self {}
    }

    #[wasm_bindgen(js_name = "loadROM")]
    pub fn load_rom(&self, name: &str, addr: u16, data: &[u8]) {
        ATARI.with_borrow_mut(|a| a.load_rom(addr, data));
        console_log!("Loaded ROM {name} @{addr:04X}");
    }

    pub fn tick(&self, loops: usize) {
        ATARI.with_borrow_mut(|a| a.tick(loops))
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
