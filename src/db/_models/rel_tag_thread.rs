use std::time::SystemTime;

#[derive(Queryable)]
pub struct RelTagThread {
    pub id: i32,
    pub tag: i32,
    pub thread: i32
}
