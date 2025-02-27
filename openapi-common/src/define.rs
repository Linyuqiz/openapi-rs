use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::pin::Pin;

pub type HttpFn<R> = Box<dyn FnOnce() -> (RequestFn, AsyncResponseFn<R>) + Send + Sync>;

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

    pub headers: Option<HashMap<String, String>>,

    pub query: Option<HashMap<String, String>>,
    pub form: Option<HashMap<String, String>>,
    pub body: Option<String>,
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
