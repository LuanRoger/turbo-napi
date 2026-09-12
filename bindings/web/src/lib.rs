use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn add(left: f64, right: f64) -> f64 {
    log("add called");
    turbo_engine::add(left, right)
}

#[wasm_bindgen]
pub fn subtract(left: f64, right: f64) -> f64 {
    turbo_engine::subtract(left, right)
}

#[wasm_bindgen]
pub fn multiply(left: f64, right: f64) -> f64 {
    turbo_engine::multiply(left, right)
}

#[wasm_bindgen]
pub fn divide(left: f64, right: f64) -> f64 {
    turbo_engine::divide(left, right)
}
