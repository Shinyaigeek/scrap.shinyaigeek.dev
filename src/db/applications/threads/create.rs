use super::super::super::_models::threads::Thread;
use super::super::super::_repositories::threads::create::{create, NewThread};
use super::super::super::connection::establish::DbConnection;
use diesel::result::Error;

pub fn create_thread(
    title: String,
    slug: String,
    content: String,
    published: bool,
    connection: DbConnection,
) -> Result<Thread, Error> {
    create(
        NewThread {
            title,
            slug,
            content,
            published,
        },
        connection,
    )
}
