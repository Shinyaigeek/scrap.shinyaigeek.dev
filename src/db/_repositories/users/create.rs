use super::super::super::_models::users::User;
use super::super::super::_schemas::users::users;
use super::super::super::connection::establish::DbConnection;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use std::time::SystemTime;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub gh_user_id: String,
}

pub fn create(thread: NewUser, connection: DbConnection) -> Result<User, Error> {
    diesel::insert_into(users::table)
        .values(&thread)
        .get_result(&connection)
}
