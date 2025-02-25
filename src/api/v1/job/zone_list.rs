use crate::common::define::{HttpFn, RequestFn, ResponseFn};
use crate::common::request::BaseRequest;
use crate::common::response::BaseResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
pub struct ZoneListResponse {}

fn request_fn() -> RequestFn {
    || BaseRequest {
        method: reqwest::Method::GET,
        uri: "/v1/jobs/zones".to_string(),
        ..Default::default()
    }
}

fn response_fn() -> ResponseFn<BaseResponse<ZoneListResponse>> {
    fn handler(response: reqwest::Response) -> anyhow::Result<BaseResponse<ZoneListResponse>> {
        Ok(BaseResponse {
            error_code: "0".to_string(),
            error_msg: "".to_string(),
            request_id: "".to_string(),
            data: Some(ZoneListResponse::default()),
        })
    }

    handler
}
