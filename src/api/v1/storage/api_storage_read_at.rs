use crate::common::define::{
    AsyncResponseFn, BaseRequest, BytesStream, HttpBuilder, HttpFn, HttpStreamBuilder, RequestFn,
};
use bytes::Bytes;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiStorageReadAtRequest {
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "Compressor")]
    pub compressor: Option<String>,
    #[serde(rename = "Offset")]
    pub offset: Option<isize>,
    #[serde(rename = "Length")]
    pub length: Option<isize>,
}

impl ApiStorageReadAtRequest {
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

    fn request_fn(self) -> RequestFn {
        let request_fn: RequestFn = Box::new(move || {
            let mut queries = HashMap::new();
            if let Some(path) = self.path {
                queries.insert("Path".to_string(), path);
            }
            if let Some(compressor) = self.compressor {
                queries.insert("Compressor".to_string(), compressor);
            }
            if let Some(offset) = self.offset {
                queries.insert("Offset".to_string(), offset.to_string());
            }
            if let Some(length) = self.length {
                queries.insert("Length".to_string(), length.to_string());
            }
            BaseRequest {
                method: Method::GET,
                uri: "/api/storage/readAt".to_string(),
                queries: Some(queries),
                ..Default::default()
            }
        });
        request_fn
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiStorageReadAtResponse {
    #[serde(rename = "Data", skip)]
    pub data: Option<Bytes>,
}

impl HttpBuilder for ApiStorageReadAtRequest {
    type Response = ApiStorageReadAtResponse;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let response_fn: AsyncResponseFn<Self::Response> = Box::new(|response: Response| {
                Box::pin(async move {
                    Ok(ApiStorageReadAtResponse {
                        data: Some(response.bytes().await?),
                    })
                })
            });
            (self.request_fn(), response_fn)
        })
    }
}

#[derive(derive_more::Debug, Default)]
pub struct ApiStorageReadAtStreamResponse {
    #[debug(skip)]
    pub stream: Option<BytesStream>,
}

impl HttpStreamBuilder for ApiStorageReadAtRequest {
    type Response = ApiStorageReadAtStreamResponse;

    fn stream_builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let response_fn: AsyncResponseFn<Self::Response> = Box::new(|response: Response| {
                Box::pin(async move {
                    Ok(ApiStorageReadAtStreamResponse {
                        stream: Some(Box::pin(response.bytes_stream())),
                    })
                })
            });
            (self.request_fn(), response_fn)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::client::OpenApiClient;
    use crate::common::config::{EndpointType, OpenApiConfig};
    use futures_util::StreamExt;
    use tracing::info;

    #[tokio::test]
    async fn test_api_storage_read_at() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let user_id = config.user_id.clone();
        let mut client = OpenApiClient::new(config).with_endpoint_type(EndpointType::Cloud);

        let http_fn = ApiStorageReadAtRequest::new()
            .with_path(format!("/{}/runner.py", user_id))
            .with_offset(0)
            .with_length(1024)
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn test_api_storage_write_at_stream() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let user_id = config.user_id.clone();
        let mut client = OpenApiClient::new(config).with_endpoint_type(EndpointType::Cloud);

        let http_fn = ApiStorageReadAtRequest::new()
            .with_path(format!("/{}/runner.py", user_id))
            .with_offset(0)
            .with_length(1024)
            .stream_builder();
        let mut response = client.send(http_fn).await?;
        while let Some(data) = response
            .stream
            .as_mut()
            .expect("stream not found")
            .next()
            .await
        {
            info!("data: {:#?}", data?);
        }

        Ok(())
    }
}
