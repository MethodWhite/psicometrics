use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
    Json,
};

use crate::community::{self, CommunityStore};
use crate::error::{AppError, AppResult};

use super::{CreatePostRequest, LikeResponse, PostResponse, PostsQuery};

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
