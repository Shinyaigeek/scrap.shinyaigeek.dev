use super::super::super::_models::threads::Thread;
use super::super::super::_schemas::threads::threads;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;

pub fn reads(connection: PgConnection) -> Vec<Thread> {
    threads::dsl::threads.load::<Thread>(&connection).unwrap()
}
