// src/lib/startup.rs

// contains all the startup and configuration logic for the application

// dependencies
use crate::routes::health_check;
use crate::telemetry::MakeRequestUuid;
use axum::{Router, http::HeaderName, routing::get};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

// struct type to represent the application, wraps an Axum Router type
pub struct Application(pub Router);

// methods for the Application type
impl Application {
    // builds the router for the application
    pub fn build() -> Self {
        // define the tracing layer
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(
                DefaultMakeSpan::new()
                    .include_headers(true)
                    .level(Level::INFO),
            )
            .on_response(DefaultOnResponse::new().include_headers(true));
        let x_request_id = HeaderName::from_static("x-request-id");

        // build the router, with state and tracing
        let router = Router::new()
            .route("/api/health_check", get(health_check))
            .fallback_service(ServeDir::new("public"))
            .layer(
                ServiceBuilder::new()
                    .layer(SetRequestIdLayer::new(
                        x_request_id.clone(),
                        MakeRequestUuid,
                    ))
                    .layer(trace_layer)
                    .layer(PropagateRequestIdLayer::new(x_request_id)),
            );

        Self(router)
    }

    // utility function to run the application until stopped, to facilitate testing
    pub async fn run_until_stopped(self, listener: TcpListener) -> Result<(), anyhow::Error> {
        axum::serve(listener, self.0).await?;

        Ok(())
    }
}
