use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use crate::model::file::FileInfo;
use bytes::Bytes;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiStorageWriteAtRequest {
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "Compressor")]
    pub compressor: Option<String>,
    #[serde(rename = "Offset")]
    pub offset: Option<isize>,
    #[serde(rename = "Length")]
    pub length: Option<isize>,
}

impl ApiStorageWriteAtRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }
    pub fn with_compressor(mut self, compressor: String) -> Self {
        self.compressor = Some(compressor);
        self
    }
    pub fn with_offset(mut self, offset: isize) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn with_length(mut self, length: isize) -> Self {
        self.length = Some(length);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiStorageWriteAtResponse {
    #[serde(rename = "File")]
    pub file: Option<FileInfo>,
}

impl HttpBuilder for ApiStorageWriteAtRequest {
    type Response = BaseResponse<ApiStorageWriteAtResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || {
                let mut queries = HashMap::new();
                if let Some(ref path) = self.path {
                    queries.insert("Path".to_string(), path.clone());
                }
                if let Some(ref compressor) = self.compressor {
                    queries.insert("Compressor".to_string(), compressor.clone());
                }
                if let Some(offset) = self.offset {
                    queries.insert("Offset".to_string(), offset.to_string());
                }
                if let Some(length) = self.length {
                    queries.insert("Length".to_string(), length.to_string());
                }
                BaseRequest {
                    method: Method::POST,
                    uri: "/api/storage/writeAt".to_string(),
                    queries: Some(queries),
                    body: Bytes::from(serde_json::to_string(&self).unwrap()),
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
    async fn test_api_storage_write_at() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let user_id = config.user_id.clone();
        let mut client = OpenApiClient::new(config).with_endpoint_type(EndpointType::Cloud);

        let http_fn = ApiStorageWriteAtRequest::new()
            .with_path(format!("/{}/runner.py", user_id))
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
