use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use crate::model::file::{Chunk, ChunkChecksum};
use bytes::Bytes;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiStorageCheckSumsFindChunkRequest {
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
    #[serde(rename = "Checksums")]
    pub chunks: Option<Vec<ChunkChecksum>>,
}

impl ApiStorageCheckSumsFindChunkRequest {
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
    pub fn with_chunks(mut self, chunks: Vec<ChunkChecksum>) -> Self {
        self.chunks = Some(chunks);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiStorageChunkCheckSumsResponse {
    #[serde(rename = "Checksums")]
    pub chunks: Option<Vec<Chunk>>,
}

impl HttpBuilder for ApiStorageCheckSumsFindChunkRequest {
    type Response = BaseResponse<ApiStorageChunkCheckSumsResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || BaseRequest {
                method: Method::POST,
                uri: "/api/storage/checksumsFindChunks".to_string(),
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
    async fn test_api_storage_check_sums_find_chunk() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let user_id = config.user_id.clone();
        let mut client = OpenApiClient::new(config).with_endpoint_type(EndpointType::Cloud);

        let http_fn = ApiStorageCheckSumsFindChunkRequest::new()
            .with_path(format!("/{}/runner.py", user_id))
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
