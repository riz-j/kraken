use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct PrintNameParams {
    name: String,
    age: Option<u32>,
}

fn print_name(params: PrintNameParams) {
    println!("Name is {}", params.name);

    if let Some(age) = params.age {
        println!("Age is {}", age);
    }
}

#[derive(Deserialize, Serialize)]
struct PrintNumberParams {
    number: i32,
}

fn print_number(params: PrintNumberParams) {
    println!("Number is {}", params.number);
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
#[allow(non_camel_case_types)]
enum RpcParams {
    print_name(PrintNameParams),
    print_number(PrintNumberParams),
}

#[derive(Deserialize, Serialize)]
struct RpcRequest {
    id: String,
    method: String,
    #[serde(rename = "params")]
    param: RpcParams,
}

macro_rules! exec_rpc {
    ($func:ident, $param:expr) => {
        match $param {
            RpcParams::$func(params) => $func(params),
            _ => eprintln!("Invalid params for method {}", stringify!($func)),
        }
    };
}

async fn handle_rpc_router(Json(payload): Json<RpcRequest>) -> impl IntoResponse {
    let rpc_req_id = payload.id;

    match payload.method.as_str() {
        "print_name" => exec_rpc!(print_name, payload.param),
        "print_number" => exec_rpc!(print_number, payload.param),
        _ => unimplemented!(),
    }

    rpc_req_id
}

pub fn rpc_router() -> Router {
    Router::new().route("/", post(handle_rpc_router))
}
