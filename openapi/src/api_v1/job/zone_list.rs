use anyhow::anyhow;
use openapi_common::define::{AsyncResponseFn, BaseRequest, BaseResponse, HttpFn, RequestFn};
use openapi_model::job::zone::Zone;
use reqwest::{Method, Response};
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
        Box::new(move || {
            let request_fn: RequestFn = Box::new(|| BaseRequest {
                method: Method::GET,
                uri: "/api/zones".to_string(),
                ..Default::default()
            });
            let response_fn: AsyncResponseFn<BaseResponse<ZoneListResponse>> =
                Box::new(|response: Response| {
                    Box::pin(async move {
                        let status = response.status();
                        if !status.is_success() {
                            return Err(anyhow!("http error: {status}"));
                        }
                        response
                            .json()
                            .await
                            .map_err(|e| anyhow!("parse json error: {e}"))
                    })
                });
            (request_fn, response_fn)
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ZoneListResponse {
    #[serde(rename = "Zones")]
    pub zones: HashMap<String, Zone>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use openapi_common::client::OpenApiClient;
    use openapi_common::client::config::OpenApiConfig;
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
