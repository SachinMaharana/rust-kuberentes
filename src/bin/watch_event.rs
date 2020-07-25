#[macro_use]
extern crate log;
use futures::TryStreamExt;

use anyhow::Result;
use kube::{
    api::{Api, ListParams},
    Client,
};

use kube_runtime::{utils::try_flatten_applied, watcher};

use k8s_openapi::api::core::v1::Event;

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "info,kube=debug");
    env_logger::init();
    let client = Client::try_default().await?;

    let events: Api<Event> = Api::all(client);
    let lp = ListParams::default();

    let mut ew = Box::pin(try_flatten_applied(watcher(events, lp)));

    while let Some(event) = ew.try_next().await? {
        info!(
            "New Event: {} (via {} {})",
            event.message.unwrap(),
            event.involved_object.kind.unwrap(),
            event.involved_object.name.unwrap()
        );
    }
    Ok(())
}
