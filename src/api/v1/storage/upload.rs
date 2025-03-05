use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use bytes::Bytes;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UploadRequest {
    #[serde(rename = "Path")]
    path: Option<String>,
    #[serde(rename = "Content")]
    content: Option<Vec<u8>>,
    #[serde(rename = "Overwrite")]
    overwrite: Option<bool>,
}

impl UploadRequest {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }
    pub fn with_content(mut self, content: Vec<u8>) -> Self {
        self.content = Some(content);
        self
    }
    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = Some(overwrite);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UploadResponse {}

impl HttpBuilder for UploadRequest {
    type Response = BaseResponse<UploadResponse>;

    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || {
                let mut queries = HashMap::new();
                if let Some(path) = &self.path {
                    queries.insert("Path".to_string(), path.clone());
                }
                if let Some(overwrite) = self.overwrite {
                    queries.insert("Overwrite".to_string(), overwrite.to_string());
                }
                let mut body = Bytes::new();
                if let Some(content) = &self.content {
                    body = Bytes::from(content.clone());
                }
                BaseRequest {
                    method: Method::POST,
                    uri: "/api/storage/upload/file".to_string(),
                    queries: Some(queries),
                    body,
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
    use crate::common::config::{EndpointType, OpenApiConfig};
    use tracing::info;

    #[tokio::test]
    async fn test_upload() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let user_id = config.user_id.clone();
        let mut client = OpenApiClient::new(config).with_endpoint_type(EndpointType::Cloud);

        let http_fn = UploadRequest::new()
            .with_path(format!("/{}/runner.py", user_id))
            .with_content("print('hello world!')".as_bytes().to_vec())
            .with_overwrite(true)
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
