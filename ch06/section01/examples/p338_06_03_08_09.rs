// http://localhost:3000/qry?id=1192&name=Tremendous1192

#[tokio::main]
async fn main() {
    // ▼ List 6-8
    // Create an Axum app with two routes:
    // 1. The root route ("/") handles HTTP GET requests with the `handler_top` function.
    // 2. The "/qry" route handles HTTP GET requests with the `handler_query` function,
    //    extracting query parameters using the `axum::extract::Query` extractor.

    let app = axum::Router::new()
        .route("/", axum::routing::get(handler_top))
        .route("/qry", axum::routing::get(handler_query));

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

// ▼ List 6-9
// Handler function for the "/qry" route, extracting query parameters and responding with formatted output
async fn handler_query(
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> String {
    format!("id:{}, name:{}.", params["id"], params["name"])
}
