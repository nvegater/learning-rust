use axum::Json;
use serde::Serialize;

// In TS: `(req, res) => res.json({ message: "Hello" })`
// In Axum: return type IS the response. No `res` object.
pub async fn health() -> &'static str {
    "ok"
}

#[derive(Serialize, Debug)]
pub struct HelloResponse {
    message: String,
}

// Axum sees `Json<T>` return → sets Content-Type: application/json automatically
pub async fn hello() -> Json<HelloResponse> {
    let response = HelloResponse {
        message: "Hello from Rust!".to_string(),
    };
    println!("Responding with: {:?}", response);
    Json(response)
}
