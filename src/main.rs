use std::net::SocketAddr;

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes = Router::new().merge(hello_routes());

    // region:    --- Start Server ---
    // 127.0.0.1:8080/hello
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
    // endregion:    --- Start Server ---
}

fn hello_routes() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("handler_hello");
    let name = params.name.as_deref().unwrap_or("World");

    Html(format!("Hello, {}!", name))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("handler_hello2");

    Html(format!("Hello, {}!", name))
}
