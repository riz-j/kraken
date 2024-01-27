use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct PrintNameParams {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct PrintNumberParams {
    number: i32,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum RpcParams {
    PrintName(PrintNameParams),
    PrintNumber(PrintNumberParams),
}

#[derive(Deserialize, Serialize)]
struct RpcRequest {
    id: String,
    method: String,
    #[serde(rename = "params")]
    param: RpcParams,
}

fn print_number(number: i32) {
    println!("Number is {}", number);
}

fn print_name(name: String) {
    println!("Name is {}", name);
}

async fn handle_rpc_router(Json(payload): Json<RpcRequest>) -> impl IntoResponse {
    let rpc_req_id = payload.id;

    match payload.method.as_str() {
        "print_name" => match payload.param {
            RpcParams::PrintName(params) => print_name(params.name),
            _ => eprintln!("Invalid params for method {}", payload.method),
        },
        "print_number" => match payload.param {
            RpcParams::PrintNumber(params) => print_number(params.number),
            _ => eprintln!("Invalid params for method {}", payload.method),
        },
        _ => unimplemented!(),
    }

    rpc_req_id
}

pub fn rpc_router() -> Router {
    Router::new().route("/", post(handle_rpc_router))
}
