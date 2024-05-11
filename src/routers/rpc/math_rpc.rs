use crate::{ctx::Ctx, mc::ModelController};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ParamsForAdd {
    pub a: f32,
    pub b: f32,
}
pub async fn add(_mc: ModelController, _ctx: Ctx, params: ParamsForAdd) -> f32 {
    params.a + params.b
}

#[derive(Deserialize)]
pub struct ParamsForSubtract {
    pub a: f32,
    pub b: f32,
}
pub async fn subtract(_mc: ModelController, _ctx: Ctx, params: ParamsForSubtract) -> f32 {
    params.a - params.b
}

#[derive(Deserialize)]
pub struct ParamsForMultiply {
    pub a: f32,
    pub b: f32,
}
pub async fn multiply(_mc: ModelController, _ctx: Ctx, params: ParamsForMultiply) -> f32 {
    params.a * params.b
}

#[derive(Deserialize)]
pub struct ParamsForDivide {
    pub a: f32,
    pub b: f32,
}
pub async fn divide(_mc: ModelController, _ctx: Ctx, params: ParamsForDivide) -> f32 {
    params.a / params.b
}
