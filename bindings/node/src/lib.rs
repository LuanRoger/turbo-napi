use napi_derive::napi;

#[napi]
pub fn fib(n: u32) -> u32 {
    turbo_engine::fib(n)
}
