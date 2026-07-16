use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
    Json,
};

use crate::community::{self, CommunityStore};
use crate::error::{AppError, AppResult};

use super::{CreateStoryRequest, IdResponse, LikeResponse, StatsResponse, StoriesQuery};

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
