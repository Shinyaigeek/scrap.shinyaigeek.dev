use super::super::super::_models::users::User;
use super::super::super::_repositories::users::read::read;
use super::super::super::connection::establish::DbConnection;
use diesel::result::Error;

pub fn signin(gh_user_id: String, connection: DbConnection) -> Option<User> {
    read(gh_user_id, connection)
}
