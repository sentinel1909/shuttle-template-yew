// src/main.rs

// dependencies
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use std::path::PathBuf;
use tower_http::services::ServeDir;

// health_check endpoint
// serves a 200 OK response with no body
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

// main function, annotated with shuttle_runtime::main, spins up an axum server
// and makes the health_check endpoint available at the /health_check route
// serves a frontend built with Yew at /
#[shuttle_runtime::main]
async fn main(
    #[shuttle_static_folder::StaticFolder(folder = "dist")] dist_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/health_check", get(health_check))
        .nest_service("/", ServeDir::new(dist_folder));

    Ok(router.into())
}
