use anyhow::Context;
use axum::Router;
use listenfd::ListenFd;

use tokio::{net::TcpListener, signal};

use crate::apps::{main_app, metrics_app};

pub async fn start_main_host() -> anyhow::Result<()> {
    let app = main_app().context("failed to create main app")?;
    start_host(app, 3000).await
}

pub async fn start_metrics_host() -> anyhow::Result<()> {
    let app = metrics_app().context("failed to create metrics app")?;
    start_host(app, 3001).await
}

async fn start_host(router: Router, port: u16) -> anyhow::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd
        .take_tcp_listener(0)
        .context("failed to take listener")?
    {
        Some(listener) => {
            listener
                .set_nonblocking(true)
                .context("failed to set nonblocking")?;
            TcpListener::from_std(listener).context("failed to convert listener")?
        }
        None => TcpListener::bind(format!("127.0.0.1:{}", port))
            .await
            .context("failed to bind to port")?,
    };
    tracing::debug!(
        "listening on {}",
        listener.local_addr().context("failed to get local addr")?
    );
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("failed to start metrics server")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
