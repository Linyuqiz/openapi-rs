use openapi_common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use openapi_model::job::job::JobInfo;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct JobListRequest {
    pub job_state: Option<String>,
    pub zone: Option<String>,
    pub page_offset: Option<isize>,
    pub page_size: Option<isize>,
}

impl JobListRequest {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_job_state(mut self, job_state: String) -> Self {
        self.job_state = Some(job_state);
        self
    }

    pub fn with_zone(mut self, zone: String) -> Self {
        self.zone = Some(zone);
        self
    }

    pub fn with_page_offset(mut self, page_offset: isize) -> Self {
        self.page_offset = Some(page_offset);
        self
    }

    pub fn with_page_size(mut self, page_size: isize) -> Self {
        self.page_size = Some(page_size);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct JobListResponse {
    #[serde(rename = "Jobs")]
    pub jobs: Vec<JobInfo>,
    #[serde(rename = "Total")]
    pub total: isize,
}

impl HttpBuilder for JobListRequest {
    type Response = BaseResponse<JobListResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || {
                let mut query_params = HashMap::new();
                if let Some(job_state) = self.job_state {
                    query_params.insert("JobState".to_string(), job_state);
                }
                if let Some(zone) = self.zone {
                    query_params.insert("Zone".to_string(), zone);
                }
                if let Some(page_offset) = self.page_offset {
                    query_params.insert("PageOffset".to_string(), page_offset.to_string());
                }
                if let Some(page_size) = self.page_size {
                    query_params.insert("PageSize".to_string(), page_size.to_string());
                }
                BaseRequest {
                    method: Method::GET,
                    uri: "/api/jobs".to_string(),
                    query_params: Some(query_params),
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
    use openapi_common::client::OpenApiClient;
    use openapi_common::client::config::OpenApiConfig;
    use tracing::info;

    #[tokio::test]
    async fn test_job_get() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?.builder();
        let mut client = OpenApiClient::new(config);

        let http_fn = JobListRequest::default().builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);
        Ok(())
    }
}
