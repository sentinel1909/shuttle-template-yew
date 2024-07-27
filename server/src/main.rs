// src/main.rs

// dependencies
use rocket::{Build, Rocket, get, routes};
use rocket::fs::FileServer;
use rocket::http::Status;

// health_check handler
#[get("/health_check")]
fn health_check() -> Status {
    Status::Ok
}

// function to create a rocket instance
fn create() -> Rocket<Build> {
    rocket::build()
        .mount("/api", routes!(health_check))
        .mount("/", FileServer::from("dist"))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = create();

    Ok(rocket.into())
}
