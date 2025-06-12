// src/bin/main.rs

// binary crate for the url-shortener-v1 project

// dependencies
use server_lib::startup::Application;
use server_lib::telemetry::{get_subscriber, init_subscriber};

// main function
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // initialize tracing
    tracing::info!("Initializing tracing...");
    let subscriber = get_subscriber(
        "shuttle-template-yew-server".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // build the application
    tracing::info!("Building the application...");
    let Application(router) = Application::build();

    Ok(router.into())
}
