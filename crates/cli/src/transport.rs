use anyhow::Result;
use tokio::net::UnixStream;
use tonic::transport::{Channel, Endpoint};
use tower::service_fn;

/// Connect to the gRPC server using either a TCP or Unix socket.
#[tracing::instrument]
pub async fn connect(addr: &str) -> Result<Channel> {
    if addr.starts_with("unix:") {
        let path = addr
            .trim_start_matches("unix:")
            .trim_start_matches("//")
            .to_string();
        let ep = Endpoint::try_from("http://[::]:50051")?;
        let channel = ep
            .connect_with_connector(service_fn(move |_| UnixStream::connect(path.clone())))
            .await?;
        Ok(channel)
    } else {
        let ep = Endpoint::try_from(format!("http://{addr}"))?;
        Ok(ep.connect().await?)
    }
}
