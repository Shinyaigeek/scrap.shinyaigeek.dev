use super::super::super::_models::comments::Comment;
use super::super::super::_schemas::comments::comments;
use super::super::super::connection::establish::DbConnection;
use diesel;
use diesel::prelude::*;
use std::time::SystemTime;

pub fn reads(connection: DbConnection) -> Vec<Comment> {
    comments::dsl::comments
        .load::<Comment>(&connection)
        .unwrap()
}
