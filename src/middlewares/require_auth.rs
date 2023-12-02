use axum::{http::Request, middleware::Next, response::Response};

pub async fn require_auth<B>(req: Request<B>, next: Next<B>) -> Response {
    println!("Logged yo!");
    next.run(req).await
}
