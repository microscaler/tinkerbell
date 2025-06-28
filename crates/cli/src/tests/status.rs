use crate::commands::status::{fetch_status, render, OutputFormat};
use crate::transport;
use api::pb::api_server::{Api, ApiServer};
use api::pb::{Empty, StatusReply, Task, TaskAck};
use tonic::{Request, Response, Status};
use tokio::sync::oneshot;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

struct MockApi;

#[tonic::async_trait]
impl Api for MockApi {
    async fn status(&self, _req: Request<Empty>) -> Result<Response<StatusReply>, Status> {
        Ok(Response::new(StatusReply { message: "ok".into() }))
    }

    async fn task_submit(&self, _req: Request<Task>) -> Result<Response<TaskAck>, Status> {
        unimplemented!()
    }

    async fn ping(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Ok(Response::new(Empty {}))
    }
}

#[tokio::test]
async fn status_plain() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        Server::builder()
            .add_service(ApiServer::new(MockApi))
            .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async { rx.await.ok(); })
            .await
            .unwrap();
    });

    let channel = transport::connect(&addr.to_string()).await.unwrap();
    let mut client = api::pb::api_client::ApiClient::new(channel);
    let reply = fetch_status(&mut client).await.unwrap();
    assert_eq!(render(reply, OutputFormat::Plain), "ok");

    let _ = tx.send(());
}

#[tokio::test]
async fn status_json() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        Server::builder()
            .add_service(ApiServer::new(MockApi))
            .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async { rx.await.ok(); })
            .await
            .unwrap();
    });

    let channel = transport::connect(&addr.to_string()).await.unwrap();
    let mut client = api::pb::api_client::ApiClient::new(channel);
    let reply = fetch_status(&mut client).await.unwrap();
    assert_eq!(render(reply, OutputFormat::Json), "{\"message\":\"ok\"}");

    let _ = tx.send(());
}
