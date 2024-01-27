use axum::{response::IntoResponse, routing::post, Json, Router};

fn print_name(name: String) {
    println!("Name is {}", name);
}

#[derive(serde::Deserialize, serde::Serialize)]
struct RpcRequest {
    method: String,
    param: String,
}

async fn handle_rpc_router(Json(payload): Json<RpcRequest>) -> impl IntoResponse {
    if payload.method == "print_name" {
        print_name(payload.param);
    }
}

pub fn rpc_router() -> Router {
    Router::new().route("/", post(handle_rpc_router))
}
