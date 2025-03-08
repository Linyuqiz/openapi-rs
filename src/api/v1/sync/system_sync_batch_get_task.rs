use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use crate::model::sync::SyncTask;
use bytes::Bytes;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SystemSyncBatchGetTaskRequest {
    #[serde(rename = "JobIds")]
    pub job_ids: Option<Vec<String>>,
}

impl SystemSyncBatchGetTaskRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_job_ids(mut self, job_ids: Vec<String>) -> Self {
        self.job_ids = Some(job_ids);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SystemSyncBatchGetTaskResponse {
    #[serde(flatten)]
    pub sync_tasks: Vec<SyncTask>,
}

impl HttpBuilder for SystemSyncBatchGetTaskRequest {
    type Response = BaseResponse<SystemSyncBatchGetTaskResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || BaseRequest {
                method: Method::POST,
                uri: "/system/sync-task/batch".to_string(),
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

        let http_fn = SystemSyncBatchGetTaskRequest::new()
            .with_job_ids(vec!["123".to_string()])
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
