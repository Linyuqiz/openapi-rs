use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use crate::model::file::ChunkChecksum;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiStorageChunkCheckSumsRequest {
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "BlockSize")]
    pub block_size: Option<isize>,
    #[serde(rename = "BeginChunkOffset")]
    pub begin_chunk_offset: Option<isize>,
    #[serde(rename = "EndChunkOffset")]
    pub end_chunk_offset: Option<isize>,
    #[serde(rename = "RollingHashType")]
    pub rolling_hash_type: Option<isize>,
}

impl ApiStorageChunkCheckSumsRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }
    pub fn with_block_size(mut self, block_size: isize) -> Self {
        self.block_size = Some(block_size);
        self
    }
    pub fn with_begin_chunk_offset(mut self, begin_chunk_offset: isize) -> Self {
        self.begin_chunk_offset = Some(begin_chunk_offset);
        self
    }
    pub fn with_end_chunk_offset(mut self, end_chunk_offset: isize) -> Self {
        self.end_chunk_offset = Some(end_chunk_offset);
        self
    }
    pub fn with_rolling_hash_type(mut self, rolling_hash_type: isize) -> Self {
        self.rolling_hash_type = Some(rolling_hash_type);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiStorageChunkCheckSumsResponse {
    #[serde(rename = "Checksums")]
    pub checksums: Option<Vec<ChunkChecksum>>,
}

impl HttpBuilder for ApiStorageChunkCheckSumsRequest {
    type Response = BaseResponse<ApiStorageChunkCheckSumsResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || {
                let mut queries = HashMap::new();
                if let Some(path) = self.path {
                    queries.insert("Path".to_string(), path);
                }
                if let Some(block_size) = self.block_size {
                    queries.insert("BlockSize".to_string(), block_size.to_string());
                }
                if let Some(begin_chunk_offset) = self.begin_chunk_offset {
                    queries.insert(
                        "BeginChunkOffset".to_string(),
                        begin_chunk_offset.to_string(),
                    );
                }
                if let Some(end_chunk_offset) = self.end_chunk_offset {
                    queries.insert("EndChunkOffset".to_string(), end_chunk_offset.to_string());
                }
                if let Some(rolling_hash_type) = self.rolling_hash_type {
                    queries.insert("RollingHashType".to_string(), rolling_hash_type.to_string());
                }
                BaseRequest {
                    method: Method::GET,
                    uri: "/api/storage/checksum".to_string(),
                    queries: Some(queries),
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
    async fn test_api_storage_chunk_check_sums() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let user_id = config.user_id.clone();
        let mut client = OpenApiClient::new(config).with_endpoint_type(EndpointType::Cloud);

        let http_fn = ApiStorageChunkCheckSumsRequest::new()
            .with_path(format!("/{}/runner.py", user_id))
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
