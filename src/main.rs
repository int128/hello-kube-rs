use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client, ResourceExt};
use kube::api::ListParams;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);

    for pod in pods.list(&ListParams::default().limit(10)).await? {
        println!("Pod {}", pod.name_any());
    }

    Ok(())
}
