use std::time::SystemTime;

#[derive(Queryable)]
pub struct Comment {
    pub id: i32,
    pub author: String,
    pub content: String,
    pub thread: i32,
    pub published_at: SystemTime,
    pub updated_at: SystemTime,
}
