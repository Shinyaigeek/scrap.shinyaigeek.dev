use std::time::SystemTime;

#[derive(Queryable)]
pub struct Tag {
    pub id: i32,
    pub tag: String,
    pub icon: String,
}
