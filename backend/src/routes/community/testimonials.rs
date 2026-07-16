use std::sync::Arc;

use axum::{
    extract::Query,
    Extension,
    Json,
};

use crate::community::{self, CommunityStore};
use crate::error::{AppError, AppResult};

use super::{CreateTestimonialRequest, IdResponse, TestimonialsQuery};

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
