use super::super::super::_models::threads::Thread;
use super::super::super::_repositories::threads::read::{
    read_all_threads, read_thread as _read_thread, read_threads as _read_threads,
};
use super::super::super::connection::establish::DbConnection;
use diesel::result::Error;

pub fn read_threads(cnt: i32, connection: DbConnection) -> Result<Vec<Thread>, Error> {
    if cnt < 0 {
        return read_all_threads(connection);
    }

    _read_threads(cnt, connection)
}

pub fn read_thread(slug: String, connection: DbConnection) -> Result<Thread, Error> {
    match _read_thread(slug, connection) {
        Ok(threads) => {
            if threads.len() == 0 {
                return Err(diesel::result::Error::NotFound);
            }
            // TODO
            Ok(threads[0].clone())
        }
        Err(_) => Err(Error::NotFound),
    }
}
