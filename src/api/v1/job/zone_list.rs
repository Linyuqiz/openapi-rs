use crate::common::define::{HttpFn, RequestFn, ResponseFn, SD};
use crate::common::request::BaseRequest;
use crate::common::response::BaseResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ZoneListRequest {}

impl ZoneListRequest {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build<T: SD>(self) -> HttpFn<T> {
        || (request_fn(), response_fn())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ZoneListResponse {}

pub fn request_fn() -> RequestFn {
    || BaseRequest {
        method: reqwest::Method::GET,
        uri: "/v1/jobs/zones".to_string(),
        ..Default::default()
    }
}

pub fn response_fn<T: SD>() -> ResponseFn<T> {
    fn handler<T: SD>(response: reqwest::Response) -> anyhow::Result<BaseResponse<T>> {
        Ok(BaseResponse {
            error_code: "0".to_string(),
            error_msg: "".to_string(),
            request_id: "".to_string(),
            data: None,
        })
    }

    handler
}
