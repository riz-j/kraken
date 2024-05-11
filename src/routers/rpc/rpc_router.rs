use crate::ctx::Ctx;
use crate::mc::ModelController;
use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, to_value, Value};
use std::convert::Infallible;

use super::country_city_rpc::get_city;
use super::country_city_rpc::get_country;
use super::country_city_rpc::list_cities;
use super::country_city_rpc::list_countries;
use super::math_rpc::add;
use super::math_rpc::divide;
use super::math_rpc::multiply;
use super::math_rpc::subtract;

#[derive(Deserialize)]
struct RpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Deserialize)]
pub struct EmptyParams {}

macro_rules! invoke {
    ($func:ident, $mc:ident, $ctx:ident, $params:ident) => {{
        let fn_params = $params.unwrap();
        let the_params = serde_json::from_value(fn_params).unwrap();
        let result = $func($mc, $ctx, the_params).await;
        serde_json::to_value(result).unwrap()
    }};
}

async fn rpc_handler(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(rpc_req): Json<RpcRequest>,
) -> Result<Json<Value>, Infallible> {
    let id = rpc_req.id;
    let method = rpc_req.method;
    let params = rpc_req.params;

    let result: serde_json::Value = match method.as_str() {
        "add" => invoke!(add, mc, ctx, params),
        "subtract" => invoke!(subtract, mc, ctx, params),
        "multiply" => invoke!(multiply, mc, ctx, params),
        "divide" => invoke!(divide, mc, ctx, params),
        "list_countries" => invoke!(list_countries, mc, ctx, params),
        "list_cities" => invoke!(list_cities, mc, ctx, params),
        "get_country" => invoke!(get_country, mc, ctx, params),
        "get_city" => invoke!(get_city, mc, ctx, params),
        _ => to_value("Method Not Found").unwrap(),
    };

    let body_response = json!({
      "id": id,
      "result": result
    });

    Ok(Json(body_response))
}

pub fn router(mc: ModelController) -> Router {
    Router::new().route("/", post(rpc_handler)).with_state(mc)
}
