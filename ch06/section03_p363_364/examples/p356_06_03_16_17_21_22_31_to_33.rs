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

// ▼ List 6-32
// Async handler function for the "/" route, responding with an HTML page
async fn handle_index() -> axum::response::Html<String> {
    // Create an array of Myform instances
    let data = [
        Myform {
            name: "taro".to_string(),
            mail: "taro@yamada".to_string(),
        },
        Myform {
            name: "hanako".to_string(),
            mail: "hanako@flower".to_string(),
        },
        Myform {
            name: "sachiko".to_string(),
            mail: "sachiko@happy".to_string(),
        },
        Myform {
            name: "jiro".to_string(),
            mail: "jiro@change".to_string(),
        },
    ];
    
    // Create a Tera template engine instance by loading templates from the "templates" directory
    let tera = tera::Tera::new("templates/*").unwrap();

    // Create a Tera context and insert data for rendering the template
    let mut context = tera::Context::new();
    context.insert("title", "Index page");
    context.insert("data", &data);

    // Render the "index.html" template with the provided context
    let output = tera.render("index.html", &context);
    
    // Respond with an HTML response containing the rendered output
    axum::response::Html(output.unwrap())
}
