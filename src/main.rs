use std::net::SocketAddr;

use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    // 127.0.0.1:8080/hello
    let routes_hello = Router::new().route(
        "/hello",
        get(handler_hello),
    );

    // region:    --- Start Server ---
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
    // endregion:    --- Start Server ---
}

async fn handler_hello() -> impl IntoResponse {
    println!("handler_hello");

    Html("Hello, <strong>World!</strong>")
}

