use std::sync::Arc;

use anyhow::Context;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod application;
mod configuration;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cargo_create_name = env!("CARGO_CRATE_NAME");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=trace,tower_http=trace,axum::rejection=trace,sqlx=trace",
                    cargo_create_name
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    let configuration =
        Arc::new(configuration::Configuration::from_env().context("Failed to load configuration")?);
    let db_connection = Arc::new(
        infrastructure::db::connections::Connections::new(&configuration.database_url)
            .await
            .context("Failed to connect to database")?,
    );
    let account_repository = Arc::new(infrastructure::db::repositories::postgres_account_repository::PostgresAccountRepository::new(db_connection.clone()));
    let create_account_use_case = Arc::new(domain::use_cases::create_account::CreateAccount::new(
        account_repository.clone(),
    ));
    let account_service = Arc::new(application::services::account_service::AccountService::new(
        create_account_use_case,
    ));
    let main_server_task = tokio::spawn(infrastructure::http::start_main_host(
        configuration.main_port,
        account_service,
    ));
    let metrics_server_task = tokio::spawn(infrastructure::http::start_metrics_host(
        configuration.metrics_port,
    ));
    tokio::select! {
        o = main_server_task => report_exit("main server", o),
        o = metrics_server_task => report_exit("metrics server", o),
    };
    Ok(())
}

pub fn report_exit(
    task_name: &str,
    outcome: Result<Result<(), impl std::fmt::Debug + std::fmt::Display>, tokio::task::JoinError>,
) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
