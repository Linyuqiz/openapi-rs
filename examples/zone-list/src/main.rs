use openapi::api_v1::job::zone_list::ZoneListRequest;
use openapi_common::client::OpenApiClient;
use openapi_common::client::config::OpenApiConfig;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().expect("failed to load .env file");
    let config = OpenApiConfig::new().load_from_env().build();
    let mut client = OpenApiClient::new(config);

    let http_fn = ZoneListRequest::new().build();
    let response = client.send(http_fn).await.expect("failed to send request");
    info!("response: {:#?}", response);
}
