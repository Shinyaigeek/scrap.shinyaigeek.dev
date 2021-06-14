pub use diesel::pg::PgConnection as DbConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> DbConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    DbConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
