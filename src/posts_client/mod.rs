mod http_impl;

#[cfg(test)]
pub mod mock_impl;

pub use http_impl::HttpPostsClient;

use async_trait::async_trait;
use serde::Deserialize;

// -- The response shape from JSONPlaceholder --
// In TS this would be: `interface Post { userId: number; id: number; title: string; body: string }`
// In Rust: derive Deserialize so serde can parse JSON into this struct automatically.
#[derive(Debug, Clone, Deserialize, serde::Serialize, PartialEq)]
pub struct Post {
    #[serde(rename = "userId")]
    pub user_id: i64,
    pub id: i64,
    pub title: String,
    pub body: String,
}

// -- Trait = interface in TS --
//
// In TS you'd do: `interface PostsClient { getPost(id: number): Promise<Post> }`
// In Rust, traits define behavior. Any struct that `impl PostsClient` can be swapped in —
// real HTTP posts_client in prod, fake/mock in tests.
// This is dependency injection without a framework.
//
// The `Send + Sync` bounds are Rust's thread-safety markers:
//   - Send  = safe to transfer between threads
//   - Sync  = safe to reference from multiple threads
// Axum's State is shared across async tasks (threads), so the trait object must be thread-safe.
// In TS/Node you never think about this because JS is single-threaded.
//
// The #[async_trait] macro expands `async fn` into the Pin<Box<dyn Future>> signature
// automatically, so we get dyn-compatible async methods without writing it by hand.
#[async_trait]
pub trait PostsClient: Send + Sync {
    async fn get_post(&self, id: u32) -> Result<Post, String>;
}
