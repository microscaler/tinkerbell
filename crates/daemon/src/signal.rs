use crossbeam::channel::{Receiver, bounded};
use signal_hook::consts::signal::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;
use std::path::Path;

/// Create a channel that receives once either SIGINT or SIGTERM.
pub fn shutdown_channel() -> anyhow::Result<Receiver<()>> {
    let (tx, rx) = bounded(1);
    let mut signals = Signals::new([SIGINT, SIGTERM])?;
    std::thread::spawn(move || {
        for _sig in signals.forever() {
            let _ = tx.send(());
            break;
        }
    });
    Ok(rx)
}

/// Start watching the given config path for changes.
/// Currently just spawns a watcher and logs events; no hot-reload logic yet.
#[allow(dead_code)]
pub fn start_watcher<P: AsRef<Path>>(path: P) -> notify::Result<notify::RecommendedWatcher> {
    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

    let mut watcher = RecommendedWatcher::new(
        |res| {
            if let Ok(event) = res {
                tracing::debug!(?event, "config changed - hot reload not yet implemented");
            }
        },
        Config::default(),
    )?;
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;
    Ok(watcher)
}
