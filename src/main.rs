mod domain;
mod errors;
mod handlers;
mod ownership;

use axum::{Router, routing::get};
use handlers::{health, hello};

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
    //
    // ⏺ That line is doing 5 things that TS does in one (res.body):
    //
    // response.into_body()   // 1. Take ownership of the body stream (move semantics — no GC)
    // .collect()         // 2. Start collecting the stream chunks into memory
    // .await             // 3. It's async — wait for all chunks to arrive
    // .unwrap()          // 4. Collecting can fail — handle the Result
    // .to_bytes()        // 5. Convert the collected data into a byte buffer
    //
    // Why each step exists:
    //
    // - .into_body() — Rust separates headers from body. The body is a stream (like a Node ReadableStream), not a buffered string. into_ means it consumes the response — ownership moves, no copy.
    // - .collect() — Streams don't buffer by default. In Node, res.body is already buffered for you. Here you explicitly say "read the whole stream into memory." This is the http_body_util::BodyExt trait you imported.
    // - .await — Same as TS. Reading from a stream is async I/O.
    // - .unwrap() — The collect can fail (connection drops, malformed chunks). In TS this would throw. Rust returns Result<T, E> — .unwrap() says "panic if it failed" (fine in tests).
    // - .to_bytes() — Converts to a contiguous Bytes buffer you can compare against.
    //
    // The core difference: In Node, the runtime silently buffers the entire response body into memory for you. Rust refuses to do that implicitly because in production, streaming a 2GB response without buffering is the correct default. You pay for what you use.
    //

    #[tokio::test]
    async fn test_health() {
        let empty_body = Body::empty();
        let test_request = Request::builder().uri("/health").body(empty_body).unwrap();
        // the curl equivalent: `curl http://localhost:3000/health`
        let response = app().oneshot(test_request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"ok");
    }

    #[tokio::test]
    async fn test_hello_json() {
        let empty_body = Body::empty();
        let test_request = Request::builder().uri("/hello").body(empty_body).unwrap();
        let response = app().oneshot(test_request).await.unwrap();

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
