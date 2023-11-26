// http://localhost:3000/1234

// ▼ List 6-17
#[tokio::main]
async fn main() {
    // ▼ List 6-25
    // Create an Axum app with a single route:
    // - "/:value" for handling HTTP GET requests with the `handle_index` function
    let app = axum::Router::new().route("/:value", axum::routing::get(handle_index));

    // Bind the server to the address "127.0.0.1:3000" and serve the app
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// ▼ List 6-26
// Async handler function for the "/:value" route, responding with an HTML page
async fn handle_index(
    axum::extract::Path(value): axum::extract::Path<usize>,
) -> axum::response::Html<String> {
    // Create a Tera template engine instance by loading templates from the "templates" directory
    let tera = tera::Tera::new("templates/*").unwrap();

    // Create a Tera context and insert data for rendering the template
    let mut context = tera::Context::new();
    context.insert("title", "Index page");
    context.insert("value", &value);

    // Render the "index.html" template with the provided context
    let output = tera.render("index.html", &context);

    // Respond with an HTML response containing the rendered output
    axum::response::Html(output.unwrap())
}
