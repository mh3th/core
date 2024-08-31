use anyhow::Context;
use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod infrastructure;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cargo_create_name = env!("CARGO_CRATE_NAME");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    cargo_create_name
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    dotenv().context("Failed to load .env file")?;
    let main_port_env_str = format!("{}_MAIN_PORT", cargo_create_name).to_uppercase();
    let main_port_env: u16 = std::env::var(main_port_env_str.to_owned())
        .context(format!("Must be set to the {} server will listen on", main_port_env_str))?
        .parse()
        .context(format!("The {} server port must be a valid u16", main_port_env_str))?;
    let metrics_port_env_str = format!("{}_METRICS_PORT", cargo_create_name).to_uppercase();
    let metrics_port_env: u16 = std::env::var(metrics_port_env_str.to_owned())
        .context(format!("Must be set to the {} server will listen on", metrics_port_env_str))?
        .parse()
        .context(format!("The {} server port must be a valid u16", metrics_port_env_str))?;
    let main_server_task = tokio::spawn(infrastructure::web::start_main_host(main_port_env));
    let metrics_server_task = tokio::spawn(infrastructure::web::start_metrics_host(metrics_port_env));
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
