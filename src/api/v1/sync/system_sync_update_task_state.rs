use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use bytes::Bytes;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SystemSyncUpdateTaskStateRequest {
    #[serde(rename = "JobId")]
    pub job_id: Option<String>,
    #[serde(rename = "FileSyncState")]
    pub file_sync_state: Option<String>,
}

impl SystemSyncUpdateTaskStateRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_job_ids(mut self, job_id: String) -> Self {
        self.job_id = Some(job_id);
        self
    }
    pub fn with_file_sync_state(mut self, file_sync_state: String) -> Self {
        self.file_sync_state = Some(file_sync_state);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SystemSyncUpdateTaskStateResponse {}

impl HttpBuilder for SystemSyncUpdateTaskStateRequest {
    type Response = BaseResponse<SystemSyncUpdateTaskStateResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || BaseRequest {
                method: Method::PATCH,
                uri: format!("/system/sync-task/{:?}/state", self.job_id),
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

        let http_fn = SystemSyncUpdateTaskStateRequest::new()
            .with_job_ids("123".to_string())
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
