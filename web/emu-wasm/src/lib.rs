use rustella::cpu;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, emu-wasm!");
}

#[derive(Default)]
#[wasm_bindgen]
pub struct ThreeVectors {
    addr1: u16,
    addr2: u16,
    addr3: u16,
}

#[wasm_bindgen]
impl ThreeVectors {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            addr1: cpu::NMI_VECTOR.into(),
            addr2: cpu::RST_VECTOR.into(),
            addr3: cpu::IRQ_VECTOR.into(),
        }
    }

    pub fn add_all(&self) -> u16 {
        set_panic_hook();
        self.addr1.wrapping_add(self.addr2).wrapping_add(self.addr3)
    }
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
