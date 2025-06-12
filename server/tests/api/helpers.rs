// tests/api/helpers.rs

// types and functions used across all integration tests

// dependencies
use reqwest::Client;
use std::env::var;
use std::io::{sink, stdout};
use std::sync::LazyLock;
use tokio::net::TcpListener;
use server_lib::startup::Application;
use server_lib::telemetry::{get_subscriber, init_subscriber};

// static constant which creates one instance of tracing
static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, sink);
        init_subscriber(subscriber);
    }
});

// struct type which models a test application
pub struct TestApp {
    pub address: String,
    pub _port: u16,
    pub client: Client,
}

// helper function which builds and returns a test application
pub async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);
    let application = Application::build();
    let listener = TcpListener::bind("localhost:0")
        .await
        .expect("Failed to obtain a listener.");
    let addr = listener.local_addr().expect("Unable to obtain a host address.");
    let port = addr.port();

    tokio::spawn(application.run_until_stopped(listener));

    // build a client to make requests
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    TestApp {
        address: format!("http://localhost:{}", port),
        _port: port,
        client,
    }
}