use serde::{Deserialize, Serialize};

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
