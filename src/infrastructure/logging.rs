//! Configuration du logging via tracing

use tracing_subscriber::{EnvFilter, fmt};

use super::config::LoggingConfig;

/// Initialise le systeme de logging
pub fn init_logging(config: &LoggingConfig) {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.level));

    let builder = fmt::Subscriber::builder()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false);

    if config.colors {
        builder.with_ansi(true).init();
    } else {
        builder.with_ansi(false).init();
    }
}
