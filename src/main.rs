use std::net::SocketAddr;

use axum::response::Html;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    // 127.0.0.1:8080/hello
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello, <strong>World!</strong>") }),
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
