// http://localhost:3000/json/0

// ▼ List 6-11
// Importing necessary serde traits for JSON serialization and deserialization
use serde::{Deserialize, Serialize};

// ▼ List 6-3
#[tokio::main]
async fn main() {
    // ▼ List 6-12
    // Create an Axum app with two routes:
    // 1. The root route ("/") handles HTTP GET requests with the `handler_top` function.
    // 2. The "/json/:id" route handles HTTP GET requests with the `handler_json` function,
    //    extracting the "id" path parameter and responding with JSON data.
    let app = axum::Router::new()
        .route("/", axum::routing::get(handler_top))
        .route("/json/:id", axum::routing::get(handler_json));

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

// ▼ List 6-13
// Define a struct Mydata with serde traits for JSON serialization and deserialization
#[derive(Serialize, Deserialize, Debug)]
struct Mydata {
    name: String,
    mail: String,
    age: u32,
}

// ▼ List 6-14
// Handler function for the "/json/:id" route, responding with JSON data based on the "id" path parameter
async fn handler_json(
    axum::extract::Path(id): axum::extract::Path<usize>,
) -> axum::Json<serde_json::Value> {
    // Sample data array
    let data: [Mydata; 3] = [
        Mydata {
            name: String::from("Taro"),
            mail: String::from("taro@yamada"),
            age: 39,
        },
        Mydata {
            name: String::from("Hanako"),
            mail: String::from("hanako@flower"),
            age: 28,
        },
        Mydata {
            name: String::from("Sachiko"),
            mail: String::from("sachiko@happy"),
            age: 17,
        },
    ];

    // Retrieve the item based on the "id" path parameter
    let item = &data[id];

    // Convert the item to JSON and respond with axum::Json
    let data = serde_json::json!(item);
    axum::Json(data)
}
