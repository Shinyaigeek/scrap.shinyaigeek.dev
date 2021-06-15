use std::time::SystemTime;

#[derive(Queryable)]
pub struct Thread {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published: bool,
    pub is_open: bool,
    pub published_at: SystemTime,
    pub updated_at: SystemTime,
}
