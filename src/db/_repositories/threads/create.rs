use super::super::super::_models::threads::Thread;
use super::super::super::_schemas::threads::threads;
use super::super::super::connection::establish::DbConnection;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use std::time::SystemTime;

#[derive(Insertable)]
#[table_name = "threads"]
pub struct NewThread {
    pub title: String,
    pub content: String,
    pub published: bool,
}

pub fn create(thread: NewThread, connection: DbConnection) -> Result<Thread, Error> {
    diesel::insert_into(threads::table)
        .values(&thread)
        .get_result(&connection)
}
