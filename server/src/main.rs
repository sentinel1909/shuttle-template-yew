// src/main.rs

// dependencies
use rocket::fs::FileServer;
use rocket::get;
use rocket::response::content::RawJson;
use rocket::response::status;
use rocket::routes;

// health_check endpoint handler
#[get("/")]
fn health_check() -> status::Accepted<RawJson<&'static str>> {
    status::Accepted(RawJson("{ \"message\": \"200 OK\" }"))
}

// main function
#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/health_check", routes![health_check])
        .mount("/", FileServer::from("dist"));

    Ok(rocket.into())
}
