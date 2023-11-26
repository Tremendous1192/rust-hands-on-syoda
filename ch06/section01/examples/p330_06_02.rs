// http://localhost:3000/

// ▼ List 6-2
#[tokio::main]
async fn main() {
    // Create an Axum app with a single route that responds with "Hello, World!" for HTTP GET requests
    let app = axum::Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }));

    // Bind the server to the address "127.0.0.1:3000" and serve the app
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
