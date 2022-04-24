use super::super::super::_models::tags::Tag;
use super::super::super::_schemas::tags::tags;
use super::super::super::connection::establish::DbConnection;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag {
    pub tag: String,
    pub icon: String,
}

pub fn create(tag: NewTag, connection: DbConnection) -> Result<Tag, Error> {
    diesel::insert_into(tags::table)
        .values(&tag)
        .get_result(&connection)
}
