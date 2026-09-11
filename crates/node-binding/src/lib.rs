use napi_derive::napi;

/// Adds 100 to the provided value.
#[napi]
pub fn plus_100(input: u32) -> u32 {
    turbo_engine::plus_100(input)
}
