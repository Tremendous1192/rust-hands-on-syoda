// http://localhost:3000/

// ▼ List 6-3
#[tokio::main]
async fn main() {
    // Create an Axum app with two routes:
    // 1. The root route ("/") handles HTTP GET requests with the `handler_top` function.
    // 2. The "/other" route handles HTTP GET requests with the `handler_other` function.
    let app = axum::Router::new()
        .route("/", axum::routing::get(handler_top))
        .route("/other", axum::routing::get(handler_other));

    // Bind the server to the address "127.0.0.1:3000" and serve the app
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler function for the root ("/") route, responding with "Hello, World!"
async fn handler_top() -> String {
    "Hello, World!".to_string()
}

// Handler function for the "/other" route, responding with "This is other page..."
async fn handler_other() -> String {
    "This is other page...".to_string()
}
