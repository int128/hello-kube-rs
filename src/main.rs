use clap::Parser;
use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client, ResourceExt};
use kube::api::ListParams;

/// Hello world
#[derive(Parser)]
struct Cli {
    /// Namespace
    #[arg(short, long)]
    namespace: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::namespaced(client, &args.namespace);

    for pod in pods.list(&ListParams::default().limit(10)).await? {
        println!("Pod {}", pod.name_any());
    }

    Ok(())
}
