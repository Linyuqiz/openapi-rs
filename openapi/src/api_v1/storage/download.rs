use openapi_common::define::{AsyncResponseFn, BaseRequest, HttpBuilder, HttpFn, RequestFn};
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DownloadRequest {
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Range")]
    pub range: Option<String>,
    // Resolver xhttp.ResponseResolver
}

impl DownloadRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_path(mut self, path: String) -> Self {
        self.path = path;
        self
    }
    pub fn with_range(mut self, range: Option<String>) -> Self {
        self.range = range;
        self
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
    #[serde(rename = "Data")]
    pub data: Vec<u8>,
}

impl HttpBuilder for DownloadRequest {
    type Response = DownloadResponse;

    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || BaseRequest {
                method: Method::GET,
                uri: "/api/storage/download".to_string(),
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
    use openapi_common::client::OpenApiClient;
    use openapi_common::config::OpenApiConfig;
    use tracing::info;

    #[tokio::test]
    async fn test_job_get() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let mut client = OpenApiClient::new(config);

        let http_fn = DownloadRequest::new()
            .with_path("test.txt".to_string())
            .with_range(Some("bytes=0-100".to_string()))
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
