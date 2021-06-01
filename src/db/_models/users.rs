#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub gh_user_id: String,
}
