mod domain;
mod errors;
mod ownership;

use axum::{Router, routing::get, Json};
use serde::Serialize;

// -- Handlers (like Express route handlers) --

// In TS: `(req, res) => res.json({ message: "Hello" })`
// In Axum: return type IS the response. No `res` object.
async fn health() -> &'static str {
    "ok"
}

// Axum sees `Json<T>` return → sets Content-Type: application/json automatically
#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello from Rust!".to_string(),
    })
}

// -- App builder (extract so tests can reuse it) --

fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/hello", get(hello))
}

// -- Entry point --

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port 3000");

    println!("Listening on http://localhost:3000");

    axum::serve(listener, app()).await.expect("Server error");
}

// -- Tests --

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt; // for `oneshot`

    // In TS you'd use supertest. In Axum, you call the router directly — no server needed.

    #[tokio::test]
    async fn test_health() {
        let response = app()
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"ok");
    }

    #[tokio::test]
    async fn test_hello_json() {
        let response = app()
            .oneshot(Request::builder().uri("/hello").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["message"], "Hello from Rust!");
    }

    #[tokio::test]
    async fn test_not_found() {
        let response = app()
            .oneshot(Request::builder().uri("/nope").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
