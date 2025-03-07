use bytes::Bytes;
use futures::Stream;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::pin::Pin;

pub type BytesStream = Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>;

pub trait HttpBuilder {
    type Response;
    fn builder(self) -> HttpFn<Self::Response>;
}

pub trait HttpStreamBuilder {
    type Response;
    fn stream_builder(self) -> HttpFn<Self::Response>;
}

pub type HttpFn<T> = Box<dyn FnOnce() -> (RequestFn, AsyncResponseFn<T>) + Send + Sync>;

pub type RequestFn = Box<dyn FnOnce() -> BaseRequest + Send + Sync>;

pub type AsyncResponseFn<T> = Box<
    dyn FnOnce(reqwest::Response) -> Pin<Box<dyn Future<Output = anyhow::Result<T>> + Send>>
        + Send
        + Sync,
>;

#[derive(Debug, Default)]
pub struct BaseRequest {
    pub method: reqwest::Method,
    pub uri: String,

    pub headers: HeaderMap<HeaderValue>,
    pub content_type: Option<String>,

    pub queries: Option<HashMap<String, String>>,
    pub form: Option<HashMap<String, String>>,
    pub body: Bytes,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BaseResponse<T> {
    #[serde(rename = "ErrorCode")]
    pub error_code: String,
    #[serde(rename = "ErrorMessage")]
    pub error_msg: String,
    #[serde(rename = "RequestID")]
    pub request_id: String,
    #[serde(rename = "Data")]
    pub data: Option<T>,
}
