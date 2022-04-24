use super::super::super::_models::rel_tag_thread::RelTagThread;
use super::super::super::_schemas::rel_tag_thread::rel_tag_thread;
use super::super::super::connection::establish::DbConnection;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;

pub fn read_thread_with_tag(tag_id: i32, connection: DbConnection) -> Result<Vec<RelTagThread>, Error> {
    rel_tag_thread::dsl::rel_tag_thread
        .filter(rel_tag_thread::tag.eq(tag_id))
        .load::<RelTagThread>(&connection)
}

pub fn read_tag_with_thread(thread_id: i32, connection: DbConnection) -> Result<Vec<RelTagThread>, Error> {
    rel_tag_thread::dsl::rel_tag_thread
        .filter(rel_tag_thread::thread.eq(thread_id))
        .load::<RelTagThread>(&connection)
}
