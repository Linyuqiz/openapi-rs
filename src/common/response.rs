use crate::common::define::SD;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct BaseResponse<T: SD> {
    pub error_code: String,
    pub error_msg: String,
    pub request_id: String,
    pub data: Option<T>,
}
