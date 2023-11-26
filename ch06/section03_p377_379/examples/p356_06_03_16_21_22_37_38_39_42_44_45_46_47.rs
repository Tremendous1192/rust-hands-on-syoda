// http://localhost:3000/

// ▼ List 6-47
#[tokio::main]
async fn main() {
    // Create an Axum app with a single route:
    // - "/" for handling HTTP GET requests with the `handle_index` function
    let app = axum::Router::new().route("/", axum::routing::get(handle_index));

    // Bind the server to the address "127.0.0.1:3000" and serve the app
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// ▼ List 6-47
// Async handler function for the "/" route, responding with an HTML page
async fn handle_index() -> axum::response::Html<String> {
    // Create a HashMap to hold parameters for rendering the template
    let mut params = std::collections::HashMap::new();
    params.insert("title", "Index page");
    params.insert("message", "This is a sample page message!");

    // Create a Handlebars template engine instance and register a template
    let mut handlebars = handlebars::Handlebars::new();
    handlebars.register_template_string("hello", include_str!("templates/index.hbs"));

    // Render the template with the provided parameters
    let template = handlebars.render("hello", &params).unwrap();

    // Respond with an HTML response containing the rendered output
    axum::response::Html(template)
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

// ▼ List 6-39
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

// ▼ List 6-42
// Custom Tera filter function named "calc"
fn calc_filter(
    _: &tera::Value,
    map: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    // Extract price and tax values from the provided HashMap
    let price = map.get("price").unwrap().as_f64().unwrap();
    let tax = map.get("tax").unwrap().as_f64().unwrap();
    // Calculate the result and return it as a formatted string
    let res = price * tax;
    Ok(tera::Value::String(format!(
        "price:{} * tax:{} = {}",
        price, tax, res
    )))
}
