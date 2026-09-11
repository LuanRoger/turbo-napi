/// Adds 100 to the provided value.
#[cfg_attr(
    all(target_family = "wasm", not(target_os = "wasi")),
    wasm_bindgen::prelude::wasm_bindgen(js_name = plus100)
)]
pub fn plus_100(input: u32) -> u32 {
    turbo_engine::plus_100(input)
}
