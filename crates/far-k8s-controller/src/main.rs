use futures::{StreamExt, TryStreamExt};
use kube::{api::Api, runtime::watcher, Client, CustomResource};
use kube::runtime::watcher::Event;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(group = "example.com", version = "v1", kind = "Foo", namespaced)]
pub struct FooSpec {
    /// Example field
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Kubernetes client
    let client = Client::try_default().await?;
    let foos: Api<Foo> = Api::all(client);

    let mut events = watcher(foos, watcher::Config::default()).boxed();
    while let Some(event) = events.try_next().await? {
        match event {
            Event::Applied(o) => println!("Applied {}", o.metadata.name.unwrap_or_default()),
            Event::Deleted(o) => println!("Deleted {}", o.metadata.name.unwrap_or_default()),
            Event::Restarted(_) => println!("Restarted"),
        }
    }
    Ok(())
}
