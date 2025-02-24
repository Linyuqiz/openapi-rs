use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseResponse<U: Deserialize> {
    pub error_code: Option<String>,
    pub error_msg: Option<String>,
    pub request_id: Option<String>,
    pub data: Option<U>,
}
