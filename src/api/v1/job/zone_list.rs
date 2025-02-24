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

    pub fn build<T: Serialize, U: Deserialize>(self) -> HttpFn<T, U> {
        || (request_fn(), response_fn())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ZoneListResponse {}

pub fn request_fn<T: Serialize>() -> RequestFn<T> {
    || BaseRequest {
        method: reqwest::Method::GET,
        uri: "/v1/jobs/zones".to_string(),
        ..Default::default()
    }
}

pub fn response_fn<U: Deserialize>() -> ResponseFn<U> {
    |response: reqwest::Response| -> anyhow::Result<BaseResponse<ZoneListResponse>> {
        Ok(serde_json::from_value(response.json()?)?)
    }
}
