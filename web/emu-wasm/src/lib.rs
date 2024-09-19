use rustella::*;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
    static ATARI: RefCell<NtscAtari> = RefCell::new(NtscAtari::default());
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
fn initialize() {
    set_panic_hook();
    console_log!("Initialized emu_wasm...");
}

#[derive(Default)]
#[wasm_bindgen]
pub struct Atari {}

#[allow(non_snake_case)]
#[wasm_bindgen]
impl Atari {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[wasm_bindgen(js_name = "loadROM")]
    pub fn load_rom(&self, addr: u16, data: &[u8]) {
        ATARI.with_borrow_mut(|a| a.load_rom(addr, data))
    }

    pub fn tick(&self) {
        ATARI.with_borrow_mut(|a| a.tick())
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
