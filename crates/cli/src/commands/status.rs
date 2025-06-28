use anyhow::Result;
use api::pb::api_client::ApiClient;
use api::pb::{Empty, StatusReply};
use tonic::transport::Channel;

/// Query the agent for its current status message.
#[tracing::instrument(level = "debug", skip(client))]
pub async fn fetch_status(client: &mut ApiClient<Channel>) -> Result<StatusReply> {
    let resp = client.status(tonic::Request::new(Empty {})).await?;
    Ok(resp.into_inner())
}

/// Format type for CLI output.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OutputFormat {
    /// Human readable plain text.
    Plain,
    /// JSON formatted output.
    Json,
}

/// Format the status reply according to the selected [`OutputFormat`].
pub fn render(reply: StatusReply, fmt: OutputFormat) -> String {
    match fmt {
        OutputFormat::Plain => reply.message,
        OutputFormat::Json => serde_json::json!({ "message": reply.message }).to_string(),
    }
}
