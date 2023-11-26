// http://localhost:3000/

// ▼ List 6-17
#[tokio::main]
async fn main() {
    // ▼ List 6-31
    // Create an Axum app with a single route:
    // - "/" for handling HTTP GET requests with the `handle_index` function
    let app = axum::Router::new().route("/", axum::routing::get(handle_index));

    // Bind the server to the address "127.0.0.1:3000" and serve the app
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// ▼ List 6-21
use serde::{Deserialize, Serialize};

// ▼ List 6-22
// Struct representing the data in the form
#[derive(Serialize, Deserialize)]
struct Myform {
    name: String,
    mail: String,
}

// ▼リスト6-37
async fn handle_index() -> axum::response::Html<String> {
    let mut tera = tera::Tera::new("templates/*").unwrap();
    tera.register_filter("hello", hello_filter);

    let mut context = tera::Context::new();
    context.insert("title", "Index page");
    context.insert("name", "山田タロー");

    let output = tera.render("index.html", &context);
    axum::response::Html(output.unwrap())
}

// ▼リスト6-36
fn hello_filter(
    value: &tera::Value,
    _: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let s = tera::try_get_value!("hello_filter", "value", String, value);
    Ok(tera::Value::String(format!("こんにちは、 {}さん！", s)))
}
