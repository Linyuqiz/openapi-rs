use crate::openapi::client::OpenApiClient;
use crate::openapi::config::OpenApiConfig;
use api::v1::job::zone_list::ZoneListRequest;
use tracing::info;

mod api;
mod common;
mod openapi;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().expect("failed to load .env file");
    let config = OpenApiConfig::new().load_from_env();
    let mut client = OpenApiClient::new(config);

    let http_fn = ZoneListRequest::new().build();
    let response = client.send(http_fn).await.expect("failed to send request");
    info!("response: {:#?}", response);
}
