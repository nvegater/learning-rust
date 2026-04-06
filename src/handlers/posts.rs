use std::sync::Arc;

use crate::posts_client::{Post, PostsClient};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

// -- Axum State + trait objects --
//
// In Express/TS you'd stash a service on `req.app.locals` or use middleware DI.
// In Axum, `State<T>` extracts shared state that was registered on the router.
//
// `Arc<dyn PostsClient>` breaks down as:
//   - Arc        = shared ownership, thread-safe reference counting (like a shared_ptr in C++)
//   - dyn        = dynamic dispatch (like calling a method on an interface in TS)
//   - PostsClient = our trait (interface)
//
// Why Arc? Axum clones State for every request. Arc makes cloning cheap (just bumps a counter).
// Why dyn? We want to swap implementations (real vs mock) without generics infecting every type.
pub type DynPostsClient = Arc<dyn PostsClient>;

// In TS: `app.get("/posts/:id", async (req, res) => { ... })`
// In Axum: extractors pull data from the request automatically based on types.
//   - Path(id)   extracts from the URL path
//   - State(posts_client) extracts the shared state
pub async fn get_post(
    State(client): State<DynPostsClient>,
    Path(id): Path<u32>,
) -> Result<Json<Post>, (StatusCode, String)> {
    // The ? operator propagates errors — like throwing in TS, but explicit.
    // .map_err converts our String error into Axum's (StatusCode, String) response tuple.
    let post = client
        .get_post(id)
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, e))?;

    Ok(Json(post))
}

// -- Unit tests with a manual mock --
//
// No mocking library needed. We just implement the trait with hardcoded responses.
// In TS you'd use jest.fn() or sinon stubs. In Rust, traits make this natural.

// The logic of these tests is that we can simulate the return values of the Post Client
// to test the handler logic.
// Basically manipulate expected results to make sure the handlers does predictable things.
#[cfg(test)]
mod tests {
    use super::*;
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request;
    use axum::routing::get;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use crate::posts_client::mock_impl::MockPostsClient;

    // Helper: build a test router with the mock posts_client
    fn mock_app() -> Router {
        let client: DynPostsClient = Arc::new(MockPostsClient);
        Router::new()
            .route("/posts/{id}", get(get_post))
            .with_state(client)
    }

    #[tokio::test]
    async fn test_get_post_returns_mocked_data() {
        let response = mock_app()
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

        assert_eq!(post.id, 1);
        assert_eq!(post.title, "Mock title");
        assert_eq!(post.user_id, 1);
    }

    #[tokio::test]
    async fn test_get_post_not_found_returns_502() {
        let response = mock_app()
            .oneshot(
                Request::builder()
                    .uri("/posts/999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Our mock returns Err → handler maps it to BAD_GATEWAY
        assert_eq!(response.status(), StatusCode::BAD_GATEWAY);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("Post 999 not found"));
    }
}
