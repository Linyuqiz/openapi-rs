use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SystemRetransmitTaskRequest {
    #[serde(rename = "JobId")]
    pub job_id: Option<String>,
}

impl SystemRetransmitTaskRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_job_ids(mut self, job_id: String) -> Self {
        self.job_id = Some(job_id);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SystemRetransmitTaskResponse {}

impl HttpBuilder for SystemRetransmitTaskRequest {
    type Response = BaseResponse<SystemRetransmitTaskResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || BaseRequest {
                method: Method::PATCH,
                uri: format!("/system/sync-task/{:?}/retransmit", self.job_id),
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
    async fn test_system_sync_retransmit_task() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let mut client = OpenApiClient::new(config).with_endpoint_type(EndpointType::Sync);

        let http_fn = SystemRetransmitTaskRequest::new()
            .with_job_ids("123".to_string())
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
