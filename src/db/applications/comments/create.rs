use super::super::super::_models::comments::Comment;
use super::super::super::_repositories::comments::create::{create, NewComment};
use super::super::super::connection::establish::DbConnection;
use diesel::result::Error;

pub fn create_comment(
    content: String,
    author: String,
    thread: i32,
    connection: DbConnection,
) -> Result<Comment, Error> {
    create(
        NewComment {
            content,
            author,
            thread,
        },
        connection,
    )
}
