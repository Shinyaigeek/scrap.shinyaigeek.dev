use super::super::super::_models::threads::Thread;
use super::super::super::_schemas::threads::threads;
use super::super::super::connection::establish::DbConnection;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use std::time::SystemTime;

pub fn read_all_threads(connection: DbConnection) -> Result<Vec<Thread>, Error> {
    threads::dsl::threads
        .order(threads::published_at.asc())
        .load::<Thread>(&connection)
}

pub fn read_threads(cnt: i32, connection: DbConnection) -> Result<Vec<Thread>, Error> {
    threads::dsl::threads
        .limit(cnt.into())
        .order(threads::published_at.asc())
        .load::<Thread>(&connection)
}

pub fn read_thread(slug: String, connection: DbConnection) -> Result<Vec<Thread>, Error> {
    threads::dsl::threads
        .filter((threads::slug.eq(slug)))
        .load::<Thread>(&connection)
}
