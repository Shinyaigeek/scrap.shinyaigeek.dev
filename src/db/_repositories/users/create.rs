use super::super::super::_models::users::User;
use super::super::super::_schemas::users::users;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub gh_user_id: String,
}

pub fn create(thread: NewUser, connection: PgConnection) -> User {
    diesel::insert_into(users::table)
        .values(&thread)
        .get_result(&connection)
        .expect("Error creating new thread")
}
