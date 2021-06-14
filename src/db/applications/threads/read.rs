use super::super::super::_models::threads::Thread;
use super::super::super::_repositories::threads::read::{
    read_all_threads, read_threads as _read_threads,
};
use super::super::super::connection::establish::DbConnection;
use diesel::result::Error;

pub fn read_threads(cnt: i32, connection: DbConnection) -> Result<Vec<Thread>, Error> {
    if cnt < 0 {
        return read_all_threads(connection);
    }

    _read_threads(cnt, connection)
}
