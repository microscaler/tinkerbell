//! Daemon runtime entry points.

pub mod config;
mod signal;

use config::Config;
use signal::shutdown_channel;

/// Initialize the daemon and return a running instance.
pub fn init(cfg: Config) -> anyhow::Result<Daemon> {
    tracing::info!("initializing daemon");
    let shutdown = shutdown_channel()?;
    Ok(Daemon { cfg, shutdown })
}

/// Daemon state returned from [`init`].
pub struct Daemon {
    cfg: Config,
    shutdown: crossbeam::channel::Receiver<()>,
}

impl Daemon {
    /// Block until a shutdown signal is received.
    pub fn run(self) -> anyhow::Result<()> {
        tracing::info!("daemon running");
        // Watch for config changes (stub).
        let _watcher = signal::start_watcher(&self.cfg.config_path).ok();
        // Wait for shutdown signal.
        let _ = self.shutdown.recv();
        tracing::info!("daemon shutdown complete");
        Ok(())
    }
}
