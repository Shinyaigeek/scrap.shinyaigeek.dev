use super::super::super::_models::threads::Thread;
use super::super::super::_schemas::threads::threads;
use diesel;
use super::super::super::connection::establish::DbConnection;
use diesel::prelude::*;
use std::time::SystemTime;

pub fn reads(connection: DbConnection) -> Vec<Thread> {
    threads::dsl::threads.load::<Thread>(&connection).unwrap()
}
