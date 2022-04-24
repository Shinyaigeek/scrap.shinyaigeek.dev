use super::super::super::_models::rel_tag_thread::RelTagThread;
use super::super::super::_schemas::rel_tag_thread::rel_tag_thread;
use super::super::super::connection::establish::DbConnection;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use std::time::SystemTime;

#[derive(Insertable)]
#[table_name = "rel_tag_thread"]
pub struct NewRelTagThread {
    pub tag_id: i32,
    pub thread_id: i32
}

pub fn create(new_rel_tag_thread: NewRelTagThread, connection: DbConnection) -> Result<RelTagThread, Error> {
    diesel::insert_into(rel_tag_thread::table)
        .values(&new_rel_tag_thread)
        .get_result(&connection)
}
