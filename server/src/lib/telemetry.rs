// src/lib/telemetry.rs

// dependencies
use axum::http::Request;
use tower_http::request_id::{MakeRequestId, RequestId};
use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};
use uuid::Uuid;

// a struct to represent a RequestUuid
#[derive(Clone)]
pub struct MakeRequestUuid;

// implementation clock to create a RequestId
impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string();

        Some(RequestId::new(request_id.parse().unwrap()))
    }
}

// get subscriber function, sets up a tracing subscriber
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

// init subscriber function, initialize the tracing subscriber
pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    // Redirect logs to subscriber
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
