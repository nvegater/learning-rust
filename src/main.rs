mod domain;
mod errors;
mod handlers;
mod ownership;
mod posts_client;

use axum::{Router, routing::get};
use handlers::{get_post, health, hello};
use posts_client::HttpPostsClient;
use std::sync::Arc;

// app() uses the real posts_client — production entry point.
// app_with_client() accepts any PostsClient — used by tests to inject mocks.
fn app() -> Router {
    app_with_client(Arc::new(HttpPostsClient::new(
        "https://jsonplaceholder.typicode.com",
    )))
}

fn app_with_client(client: handlers::posts::DynPostsClient) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/hello", get(hello))
        .route("/posts/{id}", get(get_post))
        .with_state(client)
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
    use posts_client::Post;
    use posts_client::mock_impl::MockPostsClient;
    use tower::ServiceExt;

    fn test_app() -> Router {
        app_with_client(Arc::new(MockPostsClient))
    }

    #[tokio::test]
    async fn test_health() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"ok");
    }

    #[tokio::test]
    async fn test_hello_json() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/hello")
                    .body(Body::empty())
                    .unwrap(),
            )
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

    #[tokio::test]
    async fn test_get_post_with_mock_client() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/posts/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let post: Post = serde_json::from_slice(&body).unwrap();
        assert_eq!(post.title, "Mock title");
        assert_eq!(post.id, 1);
    }
}
