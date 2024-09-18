use anyhow::Context;
use axum::{middleware, routing::get, Extension, Router};
use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, SessionManagerLayer};
use std::{future::ready, sync::Arc};
use tower_http::{compression::CompressionLayer, services::ServeDir, timeout::TimeoutLayer};

use crate::{application::services::account_service::AccountService, domain::use_cases::create_account::CreateAccountUseCase, presentation::controllers::*};

mod metrics;
mod middlewares;
mod routes;
mod servers;

pub async fn start_main_host<T>(
    port: u16,
    account_service: Arc<AccountService<T>>,
) -> anyhow::Result<()>
where
    T: CreateAccountUseCase + Sync + Send + 'static,
 {
    let app = Router::new()
        .route("/", get(index_controller::index_handler))
        .route("/register", get(register_controller::index_handler))
        .nest_service("/static", ServeDir::new("dist/static"))
        .fallback(routes::not_found)
        .route_layer(middleware::from_fn(middlewares::track_metrics))
        .layer(middlewares::tracer_layer())
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(10)))
        .layer(CompressionLayer::new())
        .layer(
            SessionManagerLayer::new(MemoryStore::default())
                .with_secure(false)
                .with_expiry(Expiry::OnInactivity(Duration::seconds(10)))
        )
        .layer(Extension(account_service));
    servers::start_host(app, port).await
}

pub async fn start_metrics_host(port: u16) -> anyhow::Result<()> {
    let recorder_handle =
        metrics::setup_metrics_recorder().context("failed to setup metrics recorder")?;
    let router = Router::new().route("/metrics", get(move || ready(recorder_handle.render())));
    servers::start_host(router, port).await
}
