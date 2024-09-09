use anyhow::Context;
use axum::{
    middleware,
    routing::get,
    Extension, Router,
};
use std::{future::ready, sync::Arc, time::Duration};
use tower_http::{compression::CompressionLayer, services::ServeDir, timeout::TimeoutLayer};

use crate::{
    application::services::account_service::AccountService,
    presentation::controllers::*,
};

mod middlewares;
mod routes;
mod servers;
mod metrics;

pub async fn start_main_host(
    port: u16,
    account_service: Arc<AccountService>,
) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(index_controller::index_handler))
        .route("/register", get(register_controller::indexx_handler))
        .nest_service("/static", ServeDir::new("dist/static"))
        .fallback(routes::not_found)
        .route_layer(middleware::from_fn(middlewares::track_metrics))
        .layer(middlewares::tracer_layer())
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(CompressionLayer::new())
        .layer(Extension(account_service));
    servers::start_host(app, port).await
}

pub async fn start_metrics_host(port: u16) -> anyhow::Result<()> {
    let recorder_handle =
        metrics::setup_metrics_recorder().context("failed to setup metrics recorder")?;
    let router = Router::new().route("/metrics", get(move || ready(recorder_handle.render())));
    servers::start_host(router, port).await
}
