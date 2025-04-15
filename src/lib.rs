mod utils;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: f64, right: f64) -> f64 {
    left + right
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}