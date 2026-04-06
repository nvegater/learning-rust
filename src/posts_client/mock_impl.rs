use async_trait::async_trait;

use super::{Post, PostsClient};

// -- Mock posts_client: implements the same trait, returns canned data --
// No mocking library needed. We just implement the trait with hardcoded responses.
// In TS you'd use jest.fn() or sinon stubs. In Rust, traits make this natural.
pub struct MockPostsClient;

#[async_trait]
impl PostsClient for MockPostsClient {
    async fn get_post(&self, id: u32) -> Result<Post, String> {
        if id == 1 {
            Ok(Post {
                user_id: 1,
                id: 1,
                title: "Mock title".to_string(),
                body: "Mock body".to_string(),
            })
        } else {
            Err(format!("Post {} not found", id))
        }
    }
}
