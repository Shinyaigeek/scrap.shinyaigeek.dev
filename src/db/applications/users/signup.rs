use super::super::super::_models::users::User;
use super::super::super::_repositories::users::create::{create, NewUser};
use super::super::super::connection::establish::DbConnection;
use diesel::result::Error;

pub fn signup(gh_user_id: String, connection: DbConnection) -> Result<User, Error> {
    create(NewUser { gh_user_id }, connection)
}
