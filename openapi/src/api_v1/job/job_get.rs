use openapi_common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use openapi_model::job::JobInfo;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct JobGetRequest {
    pub job_id: String,
}

impl JobGetRequest {
    pub fn with_job_id(mut self, job_id: String) -> Self {
        self.job_id = job_id;
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct JobGetResponse {
    #[serde(flatten)]
    pub job_info: JobInfo,
}

impl HttpBuilder for JobGetRequest {
    type Response = BaseResponse<JobGetResponse>;

    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || BaseRequest {
                method: Method::GET,
                uri: format!("/api/jobs/{}", self.job_id),
                ..Default::default()
            });
            let response_fn: AsyncResponseFn<Self::Response> =
                Box::new(|response: Response| Box::pin(async move { Ok(response.json().await?) }));
            (request_fn, response_fn)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openapi_common::client::OpenApiClient;
    use openapi_common::client::config::OpenApiConfig;
    use tracing::info;

    #[tokio::test]
    async fn test_job_get() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?.builder();
        let mut client = OpenApiClient::new(config);

        let http_fn = JobGetRequest::default()
            .with_job_id("5p6nwsYQWaw".to_string())
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);
        Ok(())
    }
}
