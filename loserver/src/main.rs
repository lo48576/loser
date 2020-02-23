//! Server for `cardina1.red`.
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// Initialize logger.
fn init_logger() {
    /// Default log filter for debug build.
    #[cfg(debug_assertions)]
    const DEFAULT_LOG_FILTER: &str = "loserver=debug";
    /// Default log filter for release build.
    #[cfg(not(debug_assertions))]
    const DEFAULT_LOG_FILTER: &str = "loserver=info";

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(DEFAULT_LOG_FILTER))
        .init();
}

fn main() {
    init_logger();
    log::info!(
        "Starting {} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
}
