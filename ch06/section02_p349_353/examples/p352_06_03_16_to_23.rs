// http://localhost:3000/

// ▼ List 6-17
#[tokio::main]
async fn main() {
    // ▼ List 6-20
    // Create an Axum app with two routes:
    // - "/" for handling HTTP GET requests with the `handle_index` function
    // - "/post" for handling HTTP POST requests with the `handle_post` function
    let app = axum::Router::new()
        .route("/", axum::routing::get(handle_index))
        .route("/post", axum::routing::post(handle_post));

    // Bind the server to the address "127.0.0.1:3000" and serve the app
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// ▼ List 6-18
// Async handler function for the "/" route, responding with an HTML page
async fn handle_index() -> axum::response::Html<String> {
    // Create a Tera template engine instance by loading templates from the "templates" directory
    let tera = tera::Tera::new("templates/*").unwrap();

    // Create a Tera context and insert data for rendering the template
    let mut context = tera::Context::new();
    context.insert("title", "Index page");
    context.insert("message", "これはサンプルです。"); // This is a sample.

    // Render the "index.html" template with the provided context
    let output = tera.render("index.html", &context);

    // Respond with an HTML response containing the rendered output
    axum::response::Html(output.unwrap())
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

// ▼ List 6-23
// Async handler function for the "/post" route, handling HTTP POST requests
async fn handle_post(axum::Form(myform): axum::Form<Myform>) -> axum::response::Html<String> {
    // Format a message using the data from the submitted form
    let msg = format!("I am {}<{}>.", myform.name, myform.mail);

    // Create a Tera template engine instance by loading templates from the "templates" directory
    let tera = tera::Tera::new("templates/*").unwrap();

    // Create a Tera context and insert data for rendering the template
    let mut context = tera::Context::new();
    context.insert("title", "Index page");
    context.insert("message", &msg);

    // Render the "index.html" template with the provided context
    let output = tera.render("index.html", &context);

    // Respond with an HTML response containing the rendered output
    axum::response::Html(output.unwrap())
}
