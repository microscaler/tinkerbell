//! API server providing gRPC and optional REST endpoints.
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{sync::Notify, task::JoinHandle};
use tonic::{transport::Server, Request, Response, Status};

pub mod pb {
    tonic::include_proto!("api");
}

use pb::api_server::{Api, ApiServer};
use pb::{Empty, StatusReply, Task, TaskAck};

struct ApiService;

#[tonic::async_trait]
impl Api for ApiService {
    async fn status(&self, _req: Request<Empty>) -> Result<Response<StatusReply>, Status> {
        Ok(Response::new(StatusReply { message: "ok".into() }))
    }

    async fn task_submit(&self, req: Request<Task>) -> Result<Response<TaskAck>, Status> {
        let id = req.into_inner().id;
        Ok(Response::new(TaskAck { id }))
    }

    async fn ping(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Ok(Response::new(Empty {}))
    }
}

async fn run_grpc(addr: SocketAddr, shutdown: impl std::future::Future<Output = ()>) -> anyhow::Result<()> {
    let svc = ApiServer::new(ApiService);
    Server::builder()
        .add_service(svc)
        .serve_with_shutdown(addr, shutdown)
        .await?
        ;
    Ok(())
}

#[cfg(feature = "rest")]
async fn run_rest(addr: SocketAddr, shutdown: impl std::future::Future<Output = ()>) -> anyhow::Result<()> {
    use axum::{routing::{get, post}, Router};
    use axum::extract::Json;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize)]
    struct TaskReq { id: String }
    #[derive(Serialize)]
    struct Ack { id: String }

    async fn ping() -> &'static str { "pong" }

    async fn status() -> &'static str { "ok" }

    async fn task(Json(task): Json<TaskReq>) -> Json<Ack> {
        Json(Ack { id: task.id })
    }

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/status", get(status))
        .route("/task", post(task));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown)
        .await?;
    Ok(())
}

/// Start the API server in the current Tokio runtime.
///
/// The returned handle resolves once the server has shut down.
pub fn start_api_server(addr: SocketAddr, notify: Arc<Notify>) -> JoinHandle<anyhow::Result<()>> {
    tokio::spawn(async move {
        let n = notify.clone();
        let grpc = run_grpc(addr, n.notified());
        #[cfg(feature = "rest")]
        let rest = run_rest(SocketAddr::new(addr.ip(), addr.port() + 1), notify.notified());
        #[cfg(feature = "rest")]
        {
            tokio::try_join!(grpc, rest).map(|_| ())
        }
        #[cfg(not(feature = "rest"))]
        {
            grpc.await
        }
    })
}
