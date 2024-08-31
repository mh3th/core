use anyhow::Context;
use axum::{extract::MatchedPath, http::Request, middleware, routing::get, Router};
use std::{future::ready, time::Duration};
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info_span;

mod middlewares;
mod routes;
mod servers;
mod utils;

pub async fn start_main_host(port: u16) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(routes::root))
        .fallback(routes::not_found)
        .route_layer(middleware::from_fn(middlewares::track_metrics))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
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
            }),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(CompressionLayer::new());
    servers::start_host(app, port).await
}

pub async fn start_metrics_host(port: u16) -> anyhow::Result<()> {
    let recorder_handle =
        utils::setup_metrics_recorder().context("failed to setup metrics recorder")?;
    let router = Router::new().route("/metrics", get(move || ready(recorder_handle.render())));
    servers::start_host(router, port).await
}
