use std::collections::HashMap;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

// ─── Data types ───────────────────────────────────────────────────────────────

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

// ─── Store ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleComment {
    pub id: String,
    pub article_slug: String,
    pub author_name: String,
    pub content: String,
    pub created_at: i64,
    pub likes: u32,
}

pub struct CommunityStore {
    forum_posts: Mutex<HashMap<String, ForumPost>>,
    testimonials: Mutex<Vec<Testimonial>>,
    stories: Mutex<Vec<UserStory>>,
    comments: Mutex<Vec<Comment>>,
    article_comments: Mutex<Vec<ArticleComment>>,
}

impl CommunityStore {
    pub fn new() -> Self {
        Self {
            forum_posts: Mutex::new(HashMap::new()),
            testimonials: Mutex::new(Vec::new()),
            stories: Mutex::new(Vec::new()),
            comments: Mutex::new(Vec::new()),
            article_comments: Mutex::new(Vec::new()),
        }
    }

    // ── Forum ──────────────────────────────────────────────────────────────

    pub fn create_post(&self, post: ForumPost) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let mut post = post;
        post.id.clone_from(&id);
        let mut posts = self.forum_posts.lock().expect("forum_posts lock poisoned");
        posts.insert(id.clone(), post);
        id
    }

    pub fn get_posts(
        &self,
        category: Option<&str>,
        page: usize,
        per_page: usize,
    ) -> Vec<ForumPost> {
        let posts = self.forum_posts.lock().expect("forum_posts lock poisoned");
        let mut all: Vec<ForumPost> = match category {
            Some(cat) if !cat.is_empty() && cat != "all" => posts
                .values()
                .filter(|p| p.category == cat)
                .cloned()
                .collect(),
            _ => posts.values().cloned().collect(),
        };
        all.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        let start = page.saturating_sub(1) * per_page;
        all.into_iter().skip(start).take(per_page).collect()
    }

    pub fn get_post(&self, id: &str) -> Option<ForumPost> {
        let posts = self.forum_posts.lock().expect("forum_posts lock poisoned");
        posts.get(id).cloned()
    }

    pub fn like_post(&self, id: &str) -> bool {
        let mut posts = self.forum_posts.lock().expect("forum_posts lock poisoned");
        if let Some(post) = posts.get_mut(id) {
            post.likes += 1;
            true
        } else {
            false
        }
    }

    pub fn add_comment(&self, post_id: &str, comment: Comment) -> bool {
        let mut comments = self.comments.lock().expect("comments lock poisoned");
        let mut comment = comment;
        comment.id = uuid::Uuid::new_v4().to_string();
        comment.post_id = post_id.to_string();
        comments.push(comment);

        // increment comment_count on post
        if let Ok(mut posts) = self.forum_posts.lock() {
            if let Some(post) = posts.get_mut(post_id) {
                post.comment_count += 1;
            }
        }
        true
    }

    pub fn get_comments(&self, post_id: &str) -> Vec<Comment> {
        let comments = self.comments.lock().expect("comments lock poisoned");
        let mut found: Vec<Comment> = comments
            .iter()
            .filter(|c| c.post_id == post_id)
            .cloned()
            .collect();
        found.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        found
    }

    // ── Testimonials ───────────────────────────────────────────────────────

    pub fn add_testimonial(&self, t: Testimonial) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let mut t = t;
        t.id.clone_from(&id);
        let mut testimonials = self.testimonials.lock().expect("testimonials lock poisoned");
        testimonials.push(t);
        id
    }

    pub fn get_testimonials(&self, page: usize, per_page: usize) -> Vec<Testimonial> {
        let testimonials = self.testimonials.lock().expect("testimonials lock poisoned");
        let mut all = testimonials.clone();
        all.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        let start = page.saturating_sub(1) * per_page;
        all.into_iter().skip(start).take(per_page).collect()
    }

    // ── Stories ────────────────────────────────────────────────────────────

    pub fn add_story(&self, s: UserStory) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let mut s = s;
        s.id.clone_from(&id);
        let mut stories = self.stories.lock().expect("stories lock poisoned");
        stories.push(s);
        id
    }

    pub fn get_stories(
        &self,
        featured_only: bool,
        page: usize,
        per_page: usize,
    ) -> Vec<UserStory> {
        let stories = self.stories.lock().expect("stories lock poisoned");
        let mut all: Vec<UserStory> = if featured_only {
            stories.iter().filter(|s| s.featured).cloned().collect()
        } else {
            stories.clone()
        };
        all.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        let start = page.saturating_sub(1) * per_page;
        all.into_iter().skip(start).take(per_page).collect()
    }

    pub fn like_story(&self, id: &str) -> bool {
        let mut stories = self.stories.lock().expect("stories lock poisoned");
        if let Some(story) = stories.iter_mut().find(|s| s.id == id) {
            story.likes += 1;
            true
        } else {
            false
        }
    }

    // ── Stats ──────────────────────────────────────────────────────────────

    pub fn get_type_stats(&self, mbti_type: &str) -> Option<serde_json::Value> {
        let mut count: u32 = 0;
        let mut total_score: f64 = 0.0;
        let role_counts: std::collections::HashMap<String, u32> = std::collections::HashMap::new();

        // Scan posts for personality type mentions
        let posts = self.forum_posts.lock().expect("forum_posts lock poisoned");
        for post in posts.values() {
            if post.personality_type.as_deref() == Some(mbti_type) {
                count += 1;
            }
        }

        // Scan stories
        let stories = self.stories.lock().expect("stories lock poisoned");
        for story in stories.iter() {
            if story.personality_type == mbti_type {
                count += 1;
            }
        }

        // Scan testimonials
        let testimonials = self.testimonials.lock().expect("testimonials lock poisoned");
        for t in testimonials.iter() {
            if t.personality_type == mbti_type {
                count += 1;
                total_score += f64::from(t.rating);
            }
        }

        if count == 0 {
            return None;
        }

        let avg_score = if count > 0 {
            total_score / f64::from(count)
        } else {
            0.0
        };

        Some(serde_json::json!({
            "count": count,
            "avg_score": (avg_score * 100.0).round() / 100.0,
            "top_roles": role_counts,
        }))
    }

    // ── Article Comments ──────────────────────────────────────────────────

    pub fn add_article_comment(&self, slug: &str, author_name: &str, content: &str) -> ArticleComment {
        let mut comments = self.article_comments.lock().expect("article_comments lock poisoned");
        let comment = ArticleComment {
            id: uuid::Uuid::new_v4().to_string(),
            article_slug: slug.to_string(),
            author_name: author_name.to_string(),
            content: content.to_string(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
            likes: 0,
        };
        comments.push(comment.clone());
        comment
    }

    pub fn get_article_comments(&self, slug: &str) -> Vec<ArticleComment> {
        let comments = self.article_comments.lock().expect("article_comments lock poisoned");
        let mut result: Vec<ArticleComment> = comments
            .iter()
            .filter(|c| c.article_slug == slug)
            .cloned()
            .collect();
        result.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        result
    }

    pub fn like_article_comment(&self, comment_id: &str) -> bool {
        let mut comments = self.article_comments.lock().expect("article_comments lock poisoned");
        if let Some(comment) = comments.iter_mut().find(|c| c.id == comment_id) {
            comment.likes += 1;
            true
        } else {
            false
        }
    }
}
