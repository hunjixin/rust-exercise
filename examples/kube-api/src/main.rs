use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Pod;
use kube::api::ListParams;
use kube::{Api, Client, ResourceExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client: Client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);
    let lp = ListParams::default();
    let mypods = pods.list(&lp).await?;
    println!("{:?}", mypods);
    Ok(())
}
