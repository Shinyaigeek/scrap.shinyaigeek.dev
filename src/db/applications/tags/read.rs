use super::super::super::_models::comments::Comment;
use super::super::super::_repositories::comments::read::read_comments as _read_comments;
use super::super::super::connection::establish::DbConnection;
use diesel::result::Error;

pub fn read_comments(thread_id: i32, connection: DbConnection) -> Result<Vec<Comment>, Error> {
    _read_comments(thread_id, connection)
}
