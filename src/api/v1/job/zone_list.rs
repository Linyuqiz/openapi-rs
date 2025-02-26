use crate::common::define::{AsyncResponseFn, HttpFn, RequestFn};
use crate::common::request::BaseRequest;
use crate::common::response::BaseResponse;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ZoneListRequest {}

impl ZoneListRequest {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> HttpFn<BaseResponse<ZoneListResponse>> {
        || (request_fn(), response_fn())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ZoneListResponse {
    #[serde(rename = "Zones")]
    pub zones: HashMap<String, Zone>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Zone {
    #[serde(rename = "HPCEndpoint")]
    pub hpc_endpoint: String,
    #[serde(rename = "StorageEndpoint")]
    pub storage_endpoint: String,
    #[serde(rename = "CloudAppEnable")]
    pub cloud_app_enable: bool,
    #[serde(rename = "SyncRunnerEndpoint")]
    pub sync_runner_endpoint: String,
}

fn request_fn() -> RequestFn {
    Box::new(|| BaseRequest {
        method: Method::GET,
        uri: "/api/zones".to_string(),
        ..Default::default()
    })
}

fn response_fn() -> AsyncResponseFn<BaseResponse<ZoneListResponse>> {
    Box::new(|response: reqwest::Response| {
        Box::pin(async move {
            let status = response.status();
            if !status.is_success() {
                return Err(anyhow::anyhow!("http error: {}", status));
            }
            let base_response: BaseResponse<ZoneListResponse> =
                response.json().await.expect("failed to parse response");
            Ok(base_response)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openapi::client::OpenApiClient;
    use crate::openapi::config::OpenApiConfig;
    use tracing::info;

    #[tokio::test]
    async fn test_zone_list() {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv().expect("failed to load .env file");
        let config = OpenApiConfig::new().load_from_env().build();
        let mut client = OpenApiClient::new(config);

        let http_fn = ZoneListRequest::new().build();
        let response = client.send(http_fn).await.expect("failed to send request");
        info!("response: {:#?}", response);
    }
}
