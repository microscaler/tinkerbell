use api::start_api_server;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let notify = Arc::new(Notify::new());
    let addr: SocketAddr = "127.0.0.1:50051".parse().unwrap();
    let handle = start_api_server(addr, notify.clone());
    tokio::signal::ctrl_c().await?;
    notify.notify_waiters();
    handle.await??;
    Ok(())
}
