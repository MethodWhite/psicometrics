use serde::{Deserialize, Serialize};

pub mod comments;
pub mod posts;
pub mod stories;
pub mod testimonials;

pub use comments::*;
pub use posts::*;
pub use stories::*;
pub use testimonials::*;

// ─── Shared Request / Response types ─────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub author_name: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub personality_type: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct PostResponse {
    pub id: String,
}

#[derive(Deserialize)]
pub struct PostsQuery {
    pub category: Option<String>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

#[derive(Deserialize)]
pub struct CreateCommentRequest {
    pub author_name: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct CreateTestimonialRequest {
    pub name: String,
    pub text: String,
    pub personality_type: String,
    pub rating: u8,
}

#[derive(Serialize)]
pub struct IdResponse {
    pub id: String,
}

#[derive(Deserialize)]
pub struct TestimonialsQuery {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

#[derive(Deserialize)]
pub struct StoriesQuery {
    pub featured: Option<bool>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

#[derive(Deserialize)]
pub struct CreateStoryRequest {
    pub title: String,
    pub content: String,
    pub author_name: String,
    pub personality_type: String,
}

#[derive(Serialize)]
pub struct LikeResponse {
    pub liked: bool,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub count: u32,
    pub avg_score: f64,
    pub top_roles: serde_json::Value,
}

#[derive(Deserialize)]
pub struct CreateArticleCommentRequest {
    pub author_name: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct ArticleCommentResponse {
    pub id: String,
    pub article_slug: String,
    pub author_name: String,
    pub content: String,
    pub created_at: i64,
    pub likes: u32,
}
