use anyhow::Result;
use futures::prelude::*;
use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, Api, Client, Config};
use kube_runtime::watcher;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::infer().await?;
    let client = Client::new(config);
    let api = Api::<Pod>::all(client);
    let watcher = watcher(api, ListParams::default());
    watcher
        .try_for_each(|e| async move {
            println!("{:?}", e);
            Ok(())
        })
        .await?;
    Ok(())
}
