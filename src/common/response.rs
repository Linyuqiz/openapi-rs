use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseResponse<T> {
    pub error_code: String,
    pub error_msg: String,
    pub request_id: String,
    pub data: Option<T>,
}
