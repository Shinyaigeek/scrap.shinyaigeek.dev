use super::super::super::_models::comments::Comment;
use super::super::super::_schemas::comments::comments;
use super::super::super::connection::establish::DbConnection;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use std::time::SystemTime;

#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    pub content: String,
    pub author: String,
    pub thread: i32,
}

pub fn create(comment: NewComment, connection: DbConnection) -> Result<Comment, Error> {
    diesel::insert_into(comments::table)
        .values(&comment)
        .get_result(&connection)
}
