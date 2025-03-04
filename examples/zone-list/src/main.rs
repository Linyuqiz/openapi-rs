use openapi_common::client::OpenApiClient;
use openapi_common::config::OpenApiConfig;
use openapi_common::define::HttpBuilder;
use tracing::info;
use openapi_rs::api_v1::job::zone_list::ZoneListRequest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv()?;
    let config = OpenApiConfig::new().load_from_env()?;
    let mut client = OpenApiClient::new(config);

    let http_fn = ZoneListRequest::new().builder();
    let response = client.send(http_fn).await?;
    info!("response: {:#?}", response);

    Ok(())
}
