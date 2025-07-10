use cli::{Cli, Commands};
use cli::commands::status::{self, OutputFormat};
use cli::transport;
use clap::Parser;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    let fmt = if cli.json { OutputFormat::Json } else { OutputFormat::Plain };

    match cli.command {
        Commands::Status => {
            let chan = transport::connect(&cli.endpoint).await?;
            let mut client = api::pb::api_client::ApiClient::new(chan);
            let reply = status::fetch_status(&mut client).await?;
            let out = status::render(reply, if cli.plain { OutputFormat::Plain } else { fmt });
            println!("{}", out);
        }
    }
    Ok(())
}
