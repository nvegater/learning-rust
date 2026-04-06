use async_trait::async_trait;

use super::{Post, PostsClient};

// -- Real implementation using reqwest (like fetch/axios in TS) --
#[derive(Clone)]
pub struct HttpPostsClient {
    base_url: String,
    client: reqwest::Client,
}

impl HttpPostsClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            // reqwest::Client pools connections — reuse it like you'd reuse an axios instance
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl PostsClient for HttpPostsClient {
    async fn get_post(&self, id: u32) -> Result<Post, String> {
        let url = format!("{}/posts/{}", self.base_url, id);

        // In TS: `const res = await fetch(url); const data = await res.json();`
        // In Rust: reqwest does the same but returns Result, not exceptions.
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?;

        if !response.status().is_success() {
            return Err(format!("API returned status: {}", response.status()));
        }

        response
            .json::<Post>()
            .await
            .map_err(|e| format!("Failed to parse JSON: {e}"))
    }
}
