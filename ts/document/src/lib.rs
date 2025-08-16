use napi_derive::napi;

#[napi]
pub fn add(left: f64, right: f64) -> f64 {
    left + right
}
