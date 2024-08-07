// src/main.rs

// dependencies
use axum::{
    http::StatusCode, response::IntoResponse, routing::get, Router,
};
use tower_http::services::ServeDir;

// health_check handler; returns a 200 OK with empty body
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

// main function
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {

    // build the router
    let api = Router::new()
        .route("/api/health_check", get(health_check))
        .nest_service("/", ServeDir::new("public"));

    // start the service
    Ok(api.into())
}
