use std::convert::Infallible;

use tokio::time::Instant;

use axum::{
    body::Body, extract::{MatchedPath, Request}, middleware::Next, response::IntoResponse, routing::Route
};
use tower::{Layer, Service};
use tower_http::{
    classify::{ClassifyResponse, MakeClassifier, ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultMakeSpan, DefaultOnResponse, HttpMakeClassifier, TraceLayer},
};
use tracing::{info_span, Span};
pub fn test() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, impl Fn(&axum::http::Request<Body>) -> Span+Clone>  {
    let a: TraceLayer<SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>= TraceLayer::new_for_http();
    let b = a.make_span_with(|request: &Request<Body>| {
        let matched_path = request
            .extensions()
            .get::<MatchedPath>()
            .map(MatchedPath::as_str);
        info_span!(
            "http_request",
            method = ?request.method(),
            matched_path,
            some_other_field = tracing::field::Empty,
        )
    });
    b
}
pub async fn track_metrics(req: Request, next: Next) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();
    let response = next.run(req).await;
    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();
    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];
    metrics::counter!("http_requests_total", &labels).increment(1);
    metrics::histogram!("http_requests_duration_seconds", &labels).record(latency);
    response
}
