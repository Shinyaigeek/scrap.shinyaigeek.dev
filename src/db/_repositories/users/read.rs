use super::super::super::_models::users::User;
use super::super::super::_schemas::users::users;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn read(gh_user_id: String, connection: PgConnection) -> Option<User> {
    users.find(gh_user_id).first(&connection)
}