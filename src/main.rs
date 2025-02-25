use crate::api::v1::job::zone_list::ZoneListResponse;
use crate::openapi::client::OpenApiClient;
use crate::openapi::config::OpenApiConfig;
use api::v1::job::zone_list::ZoneListRequest;

mod api;
mod common;
mod openapi;
mod utils;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("failed to load .env file");
    let config = OpenApiConfig::new().load_from_env();
    let mut client = OpenApiClient::new(config);

    let http_fn = ZoneListRequest::new().build::<ZoneListResponse>();
    let x = client.with_request(http_fn).call::<ZoneListResponse>();
    println!("{:?}", x);
}
