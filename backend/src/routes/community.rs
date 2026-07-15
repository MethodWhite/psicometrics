use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::community::{self, CommunityStore};
use crate::error::{AppError, AppResult};

// ─── Request / Response types ─────────────────────────────────────────────────

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

// ─── Forum routes ─────────────────────────────────────────────────────────────

pub async fn get_posts(
    Extension(store): Extension<Arc<CommunityStore>>,
    Query(query): Query<PostsQuery>,
) -> Json<Vec<community::ForumPost>> {
    let category = query.category.as_deref();
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    Json(store.get_posts(category, page, per_page))
}

pub async fn create_post(
    Extension(store): Extension<Arc<CommunityStore>>,
    Json(body): Json<CreatePostRequest>,
) -> AppResult<Json<PostResponse>> {
    if body.title.trim().is_empty() {
        return Err(AppError::BadRequest("Title cannot be empty".to_string()));
    }
    if body.content.trim().is_empty() {
        return Err(AppError::BadRequest("Content cannot be empty".to_string()));
    }

    let valid_categories = ["general", "big_five", "mbti", "enneagram"];
    if !valid_categories.contains(&body.category.as_str()) {
        return Err(AppError::BadRequest(format!(
            "Invalid category '{}'. Must be one of: {:?}",
            body.category, valid_categories
        )));
    }

    let id = store.create_post(community::ForumPost {
        id: String::new(),
        author_id: String::new(),
        author_name: body.author_name,
        title: body.title,
        content: body.content,
        category: body.category,
        personality_type: body.personality_type,
        tags: body.tags.unwrap_or_default(),
        created_at: chrono::Utc::now().timestamp(),
        likes: 0,
        comment_count: 0,
    });

    Ok(Json(PostResponse { id }))
}

pub async fn get_post(
    Extension(store): Extension<Arc<CommunityStore>>,
    Path(id): Path<String>,
) -> AppResult<Json<community::ForumPost>> {
    store
        .get_post(&id)
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))
        .map(Json)
}

pub async fn like_post(
    Extension(store): Extension<Arc<CommunityStore>>,
    Path(id): Path<String>,
) -> AppResult<Json<LikeResponse>> {
    let liked = store.like_post(&id);
    if liked {
        Ok(Json(LikeResponse { liked: true }))
    } else {
        Err(AppError::NotFound("Post not found".to_string()))
    }
}

pub async fn get_comments(
    Extension(store): Extension<Arc<CommunityStore>>,
    Path(post_id): Path<String>,
) -> Json<Vec<community::Comment>> {
    Json(store.get_comments(&post_id))
}

pub async fn create_comment(
    Extension(store): Extension<Arc<CommunityStore>>,
    Path(post_id): Path<String>,
    Json(body): Json<CreateCommentRequest>,
) -> AppResult<Json<IdResponse>> {
    // Verify post exists
    if store.get_post(&post_id).is_none() {
        return Err(AppError::NotFound("Post not found".to_string()));
    }

    if body.content.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Comment content cannot be empty".to_string(),
        ));
    }

    let comment = community::Comment {
        id: String::new(),
        post_id: post_id.clone(),
        author_name: body.author_name,
        content: body.content,
        created_at: chrono::Utc::now().timestamp(),
    };

    store.add_comment(&post_id, comment);
    Ok(Json(IdResponse {
        id: String::new(),
    }))
}

// ─── Testimonial routes ──────────────────────────────────────────────────────

pub async fn get_testimonials(
    Extension(store): Extension<Arc<CommunityStore>>,
    Query(query): Query<TestimonialsQuery>,
) -> Json<Vec<community::Testimonial>> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    Json(store.get_testimonials(page, per_page))
}

pub async fn create_testimonial(
    Extension(store): Extension<Arc<CommunityStore>>,
    Json(body): Json<CreateTestimonialRequest>,
) -> AppResult<Json<IdResponse>> {
    if body.name.trim().is_empty() {
        return Err(AppError::BadRequest("Name cannot be empty".to_string()));
    }
    if body.text.trim().is_empty() {
        return Err(AppError::BadRequest("Text cannot be empty".to_string()));
    }
    if body.rating < 1 || body.rating > 5 {
        return Err(AppError::BadRequest(
            "Rating must be between 1 and 5".to_string(),
        ));
    }

    let id = store.add_testimonial(community::Testimonial {
        id: String::new(),
        name: body.name,
        text: body.text,
        personality_type: body.personality_type,
        rating: body.rating,
        created_at: chrono::Utc::now().timestamp(),
    });

    Ok(Json(IdResponse { id }))
}

// ─── Story routes ─────────────────────────────────────────────────────────────

pub async fn get_stories(
    Extension(store): Extension<Arc<CommunityStore>>,
    Query(query): Query<StoriesQuery>,
) -> Json<Vec<community::UserStory>> {
    let featured = query.featured.unwrap_or(false);
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    Json(store.get_stories(featured, page, per_page))
}

pub async fn create_story(
    Extension(store): Extension<Arc<CommunityStore>>,
    Json(body): Json<CreateStoryRequest>,
) -> AppResult<Json<IdResponse>> {
    if body.title.trim().is_empty() {
        return Err(AppError::BadRequest("Title cannot be empty".to_string()));
    }
    if body.content.trim().is_empty() {
        return Err(AppError::BadRequest("Content cannot be empty".to_string()));
    }

    let id = store.add_story(community::UserStory {
        id: String::new(),
        title: body.title,
        content: body.content,
        author_name: body.author_name,
        personality_type: body.personality_type,
        likes: 0,
        featured: false,
        created_at: chrono::Utc::now().timestamp(),
    });

    Ok(Json(IdResponse { id }))
}

pub async fn like_story(
    Extension(store): Extension<Arc<CommunityStore>>,
    Path(id): Path<String>,
) -> AppResult<Json<LikeResponse>> {
    let liked = store.like_story(&id);
    if liked {
        Ok(Json(LikeResponse { liked: true }))
    } else {
        Err(AppError::NotFound("Story not found".to_string()))
    }
}

// ─── Type stats route ─────────────────────────────────────────────────────────

pub async fn get_type_stats(
    Extension(store): Extension<Arc<CommunityStore>>,
    Path(mbti_type): Path<String>,
) -> AppResult<Json<StatsResponse>> {
    store
        .get_type_stats(&mbti_type)
        .map(|v| {
            Json(StatsResponse {
                count: v["count"].as_u64().unwrap_or(0) as u32,
                avg_score: v["avg_score"].as_f64().unwrap_or(0.0),
                top_roles: v["top_roles"].clone(),
            })
        })
        .ok_or_else(|| AppError::NotFound(format!("No data for type '{}'", mbti_type)))
}
