use openapi_rs::api::v1::job::any_zone_list::AnyZoneListRequest;
use openapi_rs::common::client::OpenApiClient;
use openapi_rs::common::config::OpenApiConfig;
use openapi_rs::common::define::HttpBuilder;
use tracing::info;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv()?;
    let config = OpenApiConfig::new().load_from_env()?;
    let mut client = OpenApiClient::new(config);

    let http_fn = AnyZoneListRequest::new().builder();
    let response = client.send(http_fn).await?;
    info!("response: {:#?}", response);

    Ok(())
}
