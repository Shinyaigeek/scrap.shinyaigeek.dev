use super::super::super::_models::tags::Tag;
use super::super::super::_schemas::tags::tags;
use super::super::super::connection::establish::DbConnection;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use std::time::SystemTime;

pub fn read_tags(tag_id: i32, connection: DbConnection) -> Result<Vec<Tag>, Error> {
    tags::dsl::tags
        .filter(tags::id.eq(tag_id))
        .load::<Tag>(&connection)
}
