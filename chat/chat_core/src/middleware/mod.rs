mod auth;
mod request_id;
mod server_time;

use crate::User;
pub use auth::verify_token;
use axum::Router;
use axum::middleware::from_fn;
use request_id::set_request_id;
use server_time::ServerTimeLayer;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse};
use tower_http::{LatencyUnit, compression::CompressionLayer};
use tracing::Level;

const REQUEST_ID_HEADER: &str = "x-request-id";
const SERVER_TIME_HEADER: &str = "x-server-time";

pub trait TokenVerify {
    type Error: std::fmt::Debug;
    fn verify(&self, token: &str) -> Result<User, Self::Error>;
}

pub fn set_layer(app: Router) -> Router {
    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(LatencyUnit::Micros),
        );
    app.layer(
        ServiceBuilder::new()
            .layer(tracing_layer)
            .layer(CompressionLayer::new().gzip(true).br(true).deflate(true))
            .layer(from_fn(set_request_id))
            .layer(ServerTimeLayer),
    )
}
