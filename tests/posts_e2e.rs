// -- E2E test: hits the real JSONPlaceholder API --
//
// This lives in `tests/` (integration test directory). Cargo compiles each file
// in `tests/` as a separate crate — it can only use the public API of your library.
//
// In TS: this is like a test that actually calls `fetch("https://jsonplaceholder...")`.
// These tests prove the real HTTP posts_client, serde deserialization, and API contract all work.
//
// ⚠️ These tests require internet access and depend on an external service.
//    They can be flaky if the JSONPlaceholder is down. That's the tradeoff of E2E tests.

use learning_rust::posts_client::{HttpPostsClient, PostsClient};

#[tokio::test]
async fn test_real_api_returns_post_1() {
    let client = HttpPostsClient::new("https://jsonplaceholder.typicode.com");

    let post = client.get_post(1).await.expect("API call should succeed");

    // JSONPlaceholder always returns the same data for /posts/1
    assert_eq!(post.id, 1);
    assert_eq!(post.user_id, 1);
    assert!(!post.title.is_empty(), "Title should not be empty");
    assert!(!post.body.is_empty(), "Body should not be empty");
}

#[tokio::test]
async fn test_real_api_returns_error_for_invalid_id() {
    let client = HttpPostsClient::new("https://jsonplaceholder.typicode.com");

    // JSONPlaceholder returns 404 for non-existent posts
    let result = client.get_post(99999).await;

    assert!(result.is_err(), "Should fail for non-existent post");
}
