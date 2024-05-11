use serde::Deserialize;

#[derive(Deserialize)]
pub struct ParamsForAdd {
    pub a: i16,
    pub b: i16,
}
pub fn add(params: ParamsForAdd) -> i16 {
    params.a + params.b
}

#[derive(Deserialize)]
pub struct ParamsForSubtract {
    pub a: i16,
    pub b: i16,
}
pub fn subtract(params: ParamsForSubtract) -> i16 {
    params.a - params.b
}

#[derive(Deserialize)]
pub struct ParamsForMultiply {
    pub a: i16,
    pub b: i16,
}
pub fn multiply(params: ParamsForMultiply) -> i16 {
    params.a * params.b
}

#[derive(Deserialize)]
pub struct ParamsForDivide {
    pub a: i16,
    pub b: i16,
}
pub fn divide(params: ParamsForDivide) -> i16 {
    params.a / params.b
}
