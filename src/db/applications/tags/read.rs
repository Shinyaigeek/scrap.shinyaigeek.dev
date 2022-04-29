use super::super::super::_models::tags::Tag;
use super::super::super::_repositories::tags::read::{read_tag as _read_tag, read_all_tags as _read_all_tags};
use super::super::super::connection::establish::DbConnection;
use diesel::result::Error;

pub fn read_tag(tag_id: i32, connection: DbConnection) -> Result<Vec<Tag>, Error> {
    _read_tag(tag_id, connection)
}

pub fn read_all_tags(connection: DbConnection) -> Result<Vec<Tag>, Error> {
    _read_all_tags(connection)
}
