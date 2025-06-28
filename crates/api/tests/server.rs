use api::start_api_server;
use std::sync::Arc;
use tokio::sync::Notify;

#[tokio::test]
async fn start_and_shutdown() {
    let notify = Arc::new(Notify::new());
    let handle = start_api_server("127.0.0.1:0".parse().unwrap(), notify.clone());
    // allow the server to start
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    notify.notify_waiters();
    handle.await.unwrap().unwrap();
}
