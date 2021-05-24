#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

#[get("/async")]
async fn async_hello() -> &'static str {
    rocket::tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    "Hello, Rocket! I send this message 1s later"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, async_hello])
}
