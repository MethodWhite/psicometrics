use std::sync::Arc;

use axum::{
    extract::Path,
    Extension,
    Json,
};

use crate::community::{self, CommunityStore};
use crate::error::{AppError, AppResult};

use super::{ArticleCommentResponse, CreateArticleCommentRequest, CreateCommentRequest, IdResponse, LikeResponse};

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

pub async fn get_article_comments(
    Extension(store): Extension<Arc<CommunityStore>>,
    Path(slug): Path<String>,
) -> AppResult<Json<Vec<ArticleCommentResponse>>> {
    let comments = store.get_article_comments(&slug);
    Ok(Json(comments.into_iter().map(|c| ArticleCommentResponse {
        id: c.id,
        article_slug: c.article_slug,
        author_name: c.author_name,
        content: c.content,
        created_at: c.created_at,
        likes: c.likes,
    }).collect()))
}

pub async fn create_article_comment(
    Extension(store): Extension<Arc<CommunityStore>>,
    Path(slug): Path<String>,
    Json(body): Json<CreateArticleCommentRequest>,
) -> AppResult<Json<ArticleCommentResponse>> {
    if body.author_name.trim().is_empty() {
        return Err(AppError::BadRequest("El nombre es requerido".to_string()));
    }
    if body.content.trim().is_empty() {
        return Err(AppError::BadRequest("El comentario no puede estar vacío".to_string()));
    }
    if body.content.len() > 2000 {
        return Err(AppError::BadRequest("El comentario no puede exceder 2000 caracteres".to_string()));
    }

    let comment = store.add_article_comment(&slug, &body.author_name, &body.content);
    Ok(Json(ArticleCommentResponse {
        id: comment.id,
        article_slug: comment.article_slug,
        author_name: comment.author_name,
        content: comment.content,
        created_at: comment.created_at,
        likes: comment.likes,
    }))
}

pub async fn like_article_comment(
    Extension(store): Extension<Arc<CommunityStore>>,
    Path((_slug, comment_id)): Path<(String, String)>,
) -> AppResult<Json<LikeResponse>> {
    let liked = store.like_article_comment(&comment_id);
    if liked {
        Ok(Json(LikeResponse { liked: true }))
    } else {
        Err(AppError::NotFound("Comment not found".to_string()))
    }
}
