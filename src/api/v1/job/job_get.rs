use crate::common::define::{AsyncResponseFn, HttpFn, RequestFn};
use crate::common::request::BaseRequest;
use crate::common::response::BaseResponse;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct JobGetRequest {
    pub job_id: String,
}

impl JobGetRequest {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_job_id(mut self, job_id: String) -> Self {
        self.job_id = job_id;
        self
    }

    pub fn build(self) -> HttpFn<BaseResponse<JobGetResponse>> {
        || (request_fn(), response_fn())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct JobGetResponse {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AllocResource {
    #[serde(rename = "Cores")]
    pub cores: i32,
    #[serde(rename = "Resources")]
    pub memory: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Progress {
    #[serde(rename = "TotalSize")]
    pub total_size: i32,
    #[serde(rename = "Progress")]
    pub progress: i32,
}

fn request_fn() -> RequestFn {
    Box::new(|| BaseRequest {
        method: Method::GET,
        uri: "/api/jobs/%s".to_string(),
        ..Default::default()
    })
}

fn response_fn() -> AsyncResponseFn<BaseResponse<JobGetResponse>> {
    Box::new(|response: reqwest::Response| {
        Box::pin(async move {
            let status = response.status();
            if !status.is_success() {
                return Err(anyhow::anyhow!("http error: {}", status));
            }
            let base_response: BaseResponse<JobGetResponse> =
                response.json().await.expect("failed to parse response");
            Ok(base_response)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openapi::client::OpenApiClient;
    use crate::openapi::config::OpenApiConfig;
    use tracing::info;

    #[tokio::test]
    async fn test_zone_list() {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv().expect("failed to load .env file");
        let config = OpenApiConfig::new().load_from_env().build();
        let mut client = OpenApiClient::new(config);

        let http_fn = JobGetRequest::new()
            .with_job_id("5p6nwsYQWaw".to_string())
            .build();
        let response = client.send(http_fn).await.expect("failed to send request");
        info!("response: {:#?}", response);
    }
}
