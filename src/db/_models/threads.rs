use std::time::SystemTime;

#[derive(Queryable, Clone)]
pub struct Thread {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub primary_tag_id: i32,
    pub content: String,
    pub published: bool,
    pub is_open: bool,
    pub published_at: SystemTime,
    pub updated_at: SystemTime,
}
