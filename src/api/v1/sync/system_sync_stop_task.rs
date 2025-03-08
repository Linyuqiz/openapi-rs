use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use bytes::Bytes;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SystemStopTaskRequest {
    #[serde(rename = "JobId")]
    pub job_id: Option<String>,
    #[serde(rename = "Mode")]
    pub mode: Option<isize>,
}

impl SystemStopTaskRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_job_ids(mut self, job_id: String) -> Self {
        self.job_id = Some(job_id);
        self
    }
    pub fn with_mode(mut self, mode: isize) -> Self {
        self.mode = Some(mode);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SystemStopTaskResponse {}

impl HttpBuilder for SystemStopTaskRequest {
    type Response = BaseResponse<SystemStopTaskResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || BaseRequest {
                method: Method::POST,
                uri: format!("/system/sync-task/{:?}/stop", self.job_id),
                body: Bytes::from(serde_json::to_vec(&self).unwrap()),
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
    use crate::common::client::OpenApiClient;
    use crate::common::config::{EndpointType, OpenApiConfig};
    use tracing::info;

    #[tokio::test]
    async fn test_job_list() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let mut client = OpenApiClient::new(config).with_endpoint_type(EndpointType::Sync);

        let http_fn = SystemStopTaskRequest::new()
            .with_job_ids("123".to_string())
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
