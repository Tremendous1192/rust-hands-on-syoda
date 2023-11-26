// http://localhost:3000/usr/Tremendous1192

#[tokio::main]
async fn main() {
    // ▼ List 6-4
    // Create an Axum app with two routes:
    // 1. The root route ("/") handles HTTP GET requests with the `handler_top` function.
    // 2. The "/usr/:user_id" route handles HTTP GET requests with the `handler_param` function,
    //    extracting the "user_id" path parameter.

    let app = axum::Router::new()
        .route("/", axum::routing::get(handler_top))
        .route("/usr/:user_id", axum::routing::get(handler_param));

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

// ▼ List 6-5
// Handler function for the "/usr/:user_id" route, extracting the "user_id" path parameter
async fn handler_param(axum::extract::Path(user_id): axum::extract::Path<String>) -> String {
    format!("User ID: {}", user_id)
}
