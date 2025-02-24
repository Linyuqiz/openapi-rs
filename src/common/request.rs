use crate::common::define::{RequestFn, ResponseFn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRequest<T: Serialize> {
    pub method: reqwest::Method,
    pub uri: String,

    pub headers: Option<HashMap<String, String>>,

    pub query: Option<HashMap<String, String>>,
    pub form: Option<HashMap<String, String>>,
    pub body: Option<String>,
}
