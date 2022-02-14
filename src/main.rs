#![recursion_limit = "512"]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
mod components;
//mod routes;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "export function snippetTest() { console.log('Hello from JS FFI!'); }")]
extern "C" {
    fn snippetTest();
}

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    snippetTest();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
}
