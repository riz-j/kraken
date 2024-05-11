use std::convert::Infallible;

use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, to_value, Value};

use crate::mc::ModelController;

macro_rules! invoke {
    ($func:ident, $params:ident) => {{
        let fn_params = $params.unwrap();
        let the_params = serde_json::from_value(fn_params).unwrap();
        let hasil = $func(the_params);
        serde_json::to_value(hasil).unwrap()
    }};
}

#[derive(Deserialize)]
struct RpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Deserialize)]
struct ParamsForAdd {
    pub a: i16,
    pub b: i16,
}
fn add(params: ParamsForAdd) -> i16 {
    params.a + params.b
}

#[derive(Deserialize)]
struct ParamsForSubtract {
    pub a: i16,
    pub b: i16,
}
fn subtract(params: ParamsForSubtract) -> i16 {
    params.a - params.b
}

async fn rpc_handler(
    State(_mc): State<ModelController>,
    Json(rpc_req): Json<RpcRequest>,
) -> Result<Json<Value>, Infallible> {
    let id = rpc_req.id;
    let method = rpc_req.method;
    let params = rpc_req.params;

    let result: serde_json::Value = match method.as_str() {
        "add" => invoke!(add, params),
        "subtract" => invoke!(subtract, params),
        _ => to_value("nope!").unwrap(),
    };

    let body_response = json!({
      "id": id,
      "result": result
    });

    Ok(Json(body_response))
}

pub fn router(mc: ModelController) -> Router {
    Router::new()
        .route("/handler", post(rpc_handler))
        .with_state(mc)
}
