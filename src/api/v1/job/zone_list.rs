use crate::common::define::{AsyncResponseFn, HttpFn, RequestFn};
use crate::common::request::BaseRequest;
use crate::common::response::BaseResponse;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ZoneListRequest {}

impl ZoneListRequest {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> HttpFn<BaseRequest, BaseResponse<ZoneListResponse>> {
        || (request_fn(), response_fn())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ZoneListResponse {}

fn request_fn() -> RequestFn<BaseRequest> {
    Box::new(|| BaseRequest {
        method: Method::GET,
        uri: "/v1/jobs/zones".to_string(),
        ..Default::default()
    })
}

fn response_fn() -> AsyncResponseFn<BaseResponse<ZoneListResponse>> {
    Box::new(|_response: reqwest::Response| {
        Box::pin(async move {
            Ok(BaseResponse {
                error_code: "0".to_string(),
                error_msg: "".to_string(),
                request_id: "".to_string(),
                data: Some(ZoneListResponse::default()),
            })
        })
    })
}
