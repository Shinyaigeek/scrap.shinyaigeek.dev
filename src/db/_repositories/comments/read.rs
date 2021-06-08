use super::super::super::_models::comments::Comment;
use super::super::super::_schemas::comments::comments;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;

pub fn reads(connection: PgConnection) -> Vec<Comment> {
    comments::dsl::comments
        .load::<Comment>(&connection)
        .unwrap()
}
