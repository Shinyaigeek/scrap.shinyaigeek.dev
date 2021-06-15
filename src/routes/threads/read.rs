use crate::db::applications::threads::read::read_threads;
use crate::db::connection::establish::establish_connection;
use crate::routes::{HttpRequest, HttpResponse};

pub fn threads_read(req: HttpRequest) -> HttpResponse {
    let connection = establish_connection();
    let threads = read_threads(-1, connection);
    // match threads {
    //     Ok(threads) => {

    //     }
    // }

    HttpResponse {
        body: "hogehoge".to_string(),
        status: 200,
    }
}
