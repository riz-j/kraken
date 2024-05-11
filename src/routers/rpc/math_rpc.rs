use serde::Deserialize;

#[derive(Deserialize)]
pub struct ParamsForAdd {
    pub a: f32,
    pub b: f32,
}
pub fn add(params: ParamsForAdd) -> f32 {
    params.a + params.b
}

#[derive(Deserialize)]
pub struct ParamsForSubtract {
    pub a: f32,
    pub b: f32,
}
pub fn subtract(params: ParamsForSubtract) -> f32 {
    params.a - params.b
}

#[derive(Deserialize)]
pub struct ParamsForMultiply {
    pub a: f32,
    pub b: f32,
}
pub fn multiply(params: ParamsForMultiply) -> f32 {
    params.a * params.b
}

#[derive(Deserialize)]
pub struct ParamsForDivide {
    pub a: f32,
    pub b: f32,
}
pub fn divide(params: ParamsForDivide) -> f32 {
    params.a / params.b
}
