use anyhow::{Context, Ok};
use axum::{
    body::Bytes,
    extract::{MatchedPath, Request},
    http::HeaderMap,
    middleware::{self, Next},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use std::future::ready;
use std::time::Duration;
use tokio::time::Instant;
use tower::ServiceBuilder;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer};
use tracing::{info_span, Span};

use crate::endpoints;

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}


pub fn main_app() -> anyhow::Result<Router> {
    let router = Router::new()
        .route("/", get(handler))
        .route("/test", get(handler))
        .fallback(endpoints::fallback::handler)
        .route_layer(middleware::from_fn(track_metrics))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
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
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    //
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    //
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        //
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        //
                    },
                ),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(CompressionLayer::new());
    Ok(router)
}

async fn track_metrics(req: Request, next: Next) -> impl IntoResponse {
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

fn setup_metrics_recorder() -> anyhow::Result<PrometheusHandle> {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .context("failed to set buckets for metric")?
        .install_recorder()
        .context("failed to install recorder")
}

pub fn metrics_app() -> anyhow::Result<Router> {
    let recorder_handle = setup_metrics_recorder().context("failed to setup metrics recorder")?;
    let router = Router::new().route("/metrics", get(move || ready(recorder_handle.render())));
    Ok(router)
}
