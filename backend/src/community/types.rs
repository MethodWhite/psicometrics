use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumPost {
    pub id: String,
    pub author_id: String,
    pub author_name: String,
    pub title: String,
    pub content: String,
    pub category: String, // "big_five", "mbti", "enneagram", "general"
    pub personality_type: Option<String>, // e.g. "INTJ"
    pub tags: Vec<String>,
    pub created_at: i64,
    pub likes: u32,
    pub comment_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Testimonial {
    pub id: String,
    pub name: String,
    pub text: String,
    pub personality_type: String,
    pub rating: u8, // 1-5
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStory {
    pub id: String,
    pub title: String,
    pub content: String, // markdown
    pub author_name: String,
    pub personality_type: String,
    pub likes: u32,
    pub featured: bool,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub post_id: String,
    pub author_name: String,
    pub content: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleComment {
    pub id: String,
    pub article_slug: String,
    pub author_name: String,
    pub content: String,
    pub created_at: i64,
    pub likes: u32,
}
