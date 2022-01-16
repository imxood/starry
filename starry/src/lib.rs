mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
pub use app::start;

mod plugins;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, star-blog!");
}

// #[wasm_bindgen(start)]
#[wasm_bindgen]
pub fn wasm_start() {
    // wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    // set_panic_hook();
    app::start();
}
