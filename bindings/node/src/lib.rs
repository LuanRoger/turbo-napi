use napi_derive::napi;

#[napi]
pub fn add(left: f64, right: f64) -> f64 {
    turbo_engine::add(left, right)
}

#[napi]
pub fn subtract(left: f64, right: f64) -> f64 {
    turbo_engine::subtract(left, right)
}

#[napi]
pub fn multiply(left: f64, right: f64) -> f64 {
    turbo_engine::multiply(left, right)
}

#[napi]
pub fn divide(left: f64, right: f64) -> f64 {
    turbo_engine::divide(left, right)
}
