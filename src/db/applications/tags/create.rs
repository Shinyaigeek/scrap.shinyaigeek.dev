use super::super::super::_models::tags::Tag;
use super::super::super::_repositories::tags::create::{create, NewTag};
use super::super::super::connection::establish::DbConnection;
use diesel::result::Error;

pub fn create_tag(tag: String, icon: String, connection: DbConnection) -> Result<Tag, Error> {
    create(NewTag { tag, icon }, connection)
}
