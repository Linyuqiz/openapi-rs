use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use crate::model::job::JobInfo;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiJobGetRequest {
    #[serde(rename = "JobID")]
    pub job_id: Option<String>,
}

impl ApiJobGetRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_job_id(mut self, job_id: String) -> Self {
        self.job_id = Some(job_id);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiJobGetResponse {
    #[serde(flatten)]
    pub job_info: JobInfo,
}

impl HttpBuilder for ApiJobGetRequest {
    type Response = BaseResponse<ApiJobGetResponse>;

    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || {
                let mut uri = "/api/jobs".to_string();
                if let Some(job_id) = self.job_id {
                    uri += &format!("/{}", job_id);
                }
                BaseRequest {
                    method: Method::GET,
                    uri,
                    ..Default::default()
                }
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
    use crate::common::client::OpenApiClient;
    use crate::common::config::OpenApiConfig;
    use tracing::info;

    #[tokio::test]
    async fn test_api_job_get() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let mut client = OpenApiClient::new(config);

        let http_fn = ApiJobGetRequest::new()
            .with_job_id("5p6nwsYQWaw".to_string())
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
