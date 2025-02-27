use openapi_common::define::{AsyncResponseFn, BaseRequest, BaseResponse, HttpFn, RequestFn};
use openapi_model::job::zone::Zone;
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
