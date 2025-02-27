use openapi_common::define::{AsyncResponseFn, BaseRequest, BaseResponse, HttpFn, RequestFn};
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
    use openapi_common::client::OpenApiClient;
    use openapi_common::client::config::OpenApiConfig;
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
