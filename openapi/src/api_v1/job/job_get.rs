use anyhow::anyhow;
use openapi_common::define::{AsyncResponseFn, BaseRequest, BaseResponse, HttpFn, RequestFn};
use openapi_model::job::job::JobInfo;
use reqwest::{Method, Response};
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
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || BaseRequest {
                method: Method::GET,
                uri: format!("/api/jobs/{}", self.job_id),
                ..Default::default()
            });
            let response_fn: AsyncResponseFn<BaseResponse<JobGetResponse>> =
                Box::new(|response: Response| {
                    Box::pin(async move {
                        let status = response.status();
                        if !status.is_success() {
                            return Err(anyhow!("http error: {status}"));
                        }
                        response
                            .json()
                            .await
                            .map_err(|e| anyhow!("parse json error: {e}"))
                    })
                });

            (request_fn, response_fn)
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct JobGetResponse {
    #[serde(flatten)]
    pub job_info: JobInfo,
}

#[cfg(test)]
mod tests {
    use super::*;
    use openapi_common::client::OpenApiClient;
    use openapi_common::client::config::OpenApiConfig;
    use tracing::info;

    #[tokio::test]
    async fn test_job_get() {
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
