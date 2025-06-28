use daemon::config::Config;
use std::sync::Arc;
use tokio::sync::Notify;
use api::start_api_server;

fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let path = std::env::args().nth(1).unwrap_or_else(|| "config.toml".into());
    let cfg = Config::load(&path)?;

    // Start daemon
    let daemon = daemon::init(cfg)?;

    let notify = Arc::new(Notify::new());
    let rt = tokio::runtime::Runtime::new()?;
    let api_handle = rt.enter(|| start_api_server("127.0.0.1:50051".parse().unwrap(), notify.clone()));

    let res = daemon.run();

    notify.notify_waiters();
    rt.block_on(api_handle)??;
    res
}
