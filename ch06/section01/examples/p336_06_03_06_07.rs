// http://localhost:3000/usr/1192/Tremendous1192

#[tokio::main]
async fn main() {
    // ▼ List 6-6
    // Create an Axum app with two routes:
    // 1. The root route ("/") handles HTTP GET requests with the `handler_top` function.
    // 2. The "/usr/:id/:user" route handles HTTP GET requests with the `handler_param` function,
    //    extracting the "id" and "user" path parameters.
    let app = axum::Router::new()
        .route("/", axum::routing::get(handler_top))
        .route("/usr/:id/:user", axum::routing::get(handler_param));

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

// ▼ List 6-7
// Handler function for the "/usr/:id/:user" route, extracting the "id" and "user" path parameters
async fn handler_param(
    axum::extract::Path((id, user)): axum::extract::Path<(usize, String)>,
) -> String {
    format!("User ID:{}. Name:{}.", id, user)
}
