use axum::{response::IntoResponse, routing::post, Json, Router};

fn print_number(number: String) {
    println!("Number is {}", number);
}
fn print_name(name: String) {
    println!("Name is {}", name);
}

#[derive(serde::Deserialize, serde::Serialize)]
struct RpcRequest {
    id: String,
    method: String,
    param: String,
}

async fn handle_rpc_router(Json(payload): Json<RpcRequest>) -> impl IntoResponse {
    let rpc_req_id = payload.id;

    match payload.method.as_str() {
        "print_name" => print_name(payload.param),
        "print_number" => print_number(payload.param),
        _ => unimplemented!(),
    }

    rpc_req_id
}

pub fn rpc_router() -> Router {
    Router::new().route("/", post(handle_rpc_router))
}
