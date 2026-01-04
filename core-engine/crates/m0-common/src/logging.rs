
use tracing_subscriber::{fmt, EnvFilter};

pub fn init(service_name: &str) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // Default to JSON logs in CI, pretty logs locally.
    let json = std::env::var("M0_LOG_JSON").ok().as_deref() == Some("1");
    if json {
        fmt().with_env_filter(filter).json().with_current_span(true).with_target(true).init();
    } else {
        fmt().with_env_filter(filter).with_target(true).with_thread_ids(true).init();
    }

    tracing::info!(service = service_name, "logging initialized");
}
