use super::super::super::_models::users::User;
use super::super::super::_schemas::users::users;
use diesel;
use super::super::super::connection::establish::DbConnection;
use diesel::prelude::*;

pub fn read(gh_user_id: String, connection: DbConnection) -> Option<User> {
    let res: Result<User, diesel::result::Error> =
        users::dsl::users.find(gh_user_id).first(&connection);
    match res {
        Ok(res) => Some(res),
        Err(_) => None,
    }
}
