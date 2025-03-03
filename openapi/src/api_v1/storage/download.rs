use bytes::Bytes;
use openapi_common::define::{
    AsyncResponseFn, BaseRequest, BytesStream, HttpBuilder, HttpFn, HttpStreamBuilder,
    RequestFn,
};
use regex::Regex;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DownloadRequest {
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "RangeStart")]
    pub range_start: Option<isize>,
    #[serde(rename = "RangeEnd")]
    pub range_end: Option<isize>,
}

impl DownloadRequest {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }
    pub fn with_range_start(mut self, range_start: isize) -> Self {
        self.range_start = Some(range_start);
        self
    }
    pub fn with_range_end(mut self, range_end: isize) -> Self {
        self.range_end = Some(range_end);
        self
    }

    fn request_fn(self) -> RequestFn {
        let request_fn: RequestFn = Box::new(move || {
            let mut queries = HashMap::new();
            if let Some(path) = &self.path {
                queries.insert("Path".to_string(), path.clone());
            }
            if let Some(range_start) = self.range_start {
                if let Some(range_end) = self.range_end {
                    queries.insert(
                        "Range".to_string(),
                        format!("bytes={}-{}", range_start, range_end),
                    );
                }
            }
            BaseRequest {
                method: Method::GET,
                uri: "/api/storage/download".to_string(),
                queries: Some(queries),
                ..Default::default()
            }
        });
        request_fn
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DownloadResponse {
    #[serde(rename = "FileName")]
    pub file_name: String,
    #[serde(rename = "FileType")]
    pub file_type: String,
    #[serde(rename = "FileSize")]
    pub file_size: isize,
    #[serde(rename = "Data", skip)]
    pub data: Option<Bytes>,
}

impl HttpBuilder for DownloadRequest {
    type Response = DownloadResponse;

    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let response_fn: AsyncResponseFn<Self::Response> = Box::new(|response: Response| {
                Box::pin(async move {
                    let mut download_response = DownloadResponse::default();
                    let file_name_regex = Regex::new(r#"attachment; filename="(.*?)""#)?;
                    download_response.file_name = response
                        .headers()
                        .get("Content-Disposition")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| {
                            file_name_regex
                                .captures(s)
                                .and_then(|caps| caps.get(1))
                                .map(|m| m.as_str().to_owned())
                        })
                        .unwrap();
                    download_response.file_type = response
                        .headers()
                        .get("Content-Type")
                        .unwrap()
                        .to_str()?
                        .to_owned();
                    download_response.file_size = response
                        .headers()
                        .get("Content-Length")
                        .unwrap()
                        .to_str()?
                        .parse::<isize>()?;
                    download_response.data = Some(response.bytes().await?);

                    Ok(download_response)
                })
            });
            (self.request_fn(), response_fn)
        })
    }
}

// #[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[derive(Default, derive_more::Debug)]
// #[serde(default)]
pub struct DownloadStreamResponse {
    #[debug(skip)]
    pub stream: Option<BytesStream>,
}

impl HttpStreamBuilder for DownloadRequest {
    type Response = DownloadStreamResponse;

    fn stream_builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let response_fn: AsyncResponseFn<Self::Response> = Box::new(|response: Response| {
                Box::pin(async move {
                    Ok(DownloadStreamResponse {
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
    use openapi_common::client::OpenApiClient;
    use openapi_common::config::{EndpointType, OpenApiConfig};
    use tracing::info;

    #[tokio::test]
    async fn test_download() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let user_id = config.user_id.clone();
        let mut client = OpenApiClient::new(config).with_endpoint_type(EndpointType::Cloud);

        let http_fn = DownloadRequest::new()
            .with_path(format!("{}/download", user_id))
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn test_download_stream() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let user_id = config.user_id.clone();
        let mut client = OpenApiClient::new(config).with_endpoint_type(EndpointType::Cloud);

        let http_fn = DownloadRequest::new()
            .with_path(format!("{}/download", user_id))
            .stream_builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
