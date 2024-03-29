use super::super::super::_models::comments::Comment;
use super::super::super::_schemas::comments::comments;
use super::super::super::connection::establish::DbConnection;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use std::time::SystemTime;

pub fn read_comments(thread_id: i32, connection: DbConnection) -> Result<Vec<Comment>, Error> {
    comments::dsl::comments
        .filter(comments::thread.eq(thread_id))
        .load::<Comment>(&connection)
}
