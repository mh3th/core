use anyhow::Context;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};

pub fn setup_metrics_recorder() -> anyhow::Result<PrometheusHandle> {
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
