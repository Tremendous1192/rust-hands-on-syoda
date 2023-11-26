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

// ▼リスト6-34
// Async handler function for the "/" route, responding with an HTML page
async fn handle_index() -> axum::response::Html<String> {
    // Create a HashMap with user names as keys and tuples containing email and age as values
    let mut map = std::collections::HashMap::new();
    map.insert("taro", ("taro@yamada", 39));
    map.insert("hanako", ("hanako@flower", 28));
    map.insert("sachiko", ("sachiko@happy", 17));

    // Create a Tera template engine instance by loading templates from the "templates" directory
    let tera = tera::Tera::new("templates/*").unwrap();

    // Create a Tera context and insert data for rendering the template
    let mut context = tera::Context::new();
    context.insert("title", "Index page");
    context.insert("data", &map);

    // Render the "index.html" template with the provided context
    let output = tera.render("index.html", &context);

    // Respond with an HTML response containing the rendered output
    axum::response::Html(output.unwrap())
}
