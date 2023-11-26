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

// ▼リスト6-40
// Async handler function for the "/" route, responding with an HTML page
async fn handle_index() -> axum::response::Html<String> {
    // Create a Tera template engine instance by loading templates from the "templates" directory
    let mut tera = tera::Tera::new("templates/*").unwrap();
    // Register a custom Tera filter named "sample" with the function `sample_filter`
    tera.register_filter("sample", sample_filter);

    // Create a Tera context and insert data for rendering the template
    let mut context = tera::Context::new();
    context.insert("title", "Index page");
    context.insert("id", &1);

    // Render the "index.html" template with the provided context
    let output = tera.render("index.html", &context);

    // Respond with an HTML response containing the rendered output
    axum::response::Html(output.unwrap())
}

// ▼リスト6-39
// Custom Tera filter function named "sample"
fn sample_filter(
    value: &tera::Value,
    _: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    // Define an array of data tuples
    let data = [
        ("taro", "taro@yamada", 39, "male"),
        ("hanako", "hanako@flower", 28, "female"),
        ("sachiko", "sachiko@happy", 17, "female"),
        ("jiro", "jiro@change", 6, "male"),
    ];
    // Extract a usize value from the Tera value
    let n = tera::try_get_value!("sample_filter", "value", usize, value);
    // Access the corresponding item from the data array
    let item = data[n];
    // Return a new Tera value containing a formatted string
    Ok(tera::Value::String(format!(
        "{}({},{})<{}>.",
        item.0, item.3, item.2, item.1
    )))
}
