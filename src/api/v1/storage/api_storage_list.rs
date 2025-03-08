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
pub struct ApiStorageListRequest {
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "FilterRegexp")]
    pub filter_regexp: Option<String>,
    #[serde(rename = "FilterRegexpList")]
    pub filter_regexp_list: Option<Vec<String>>,
    #[serde(rename = "PageOffset")]
    pub page_offset: Option<isize>,
    #[serde(rename = "PageSize")]
    pub page_size: Option<isize>,
}

impl ApiStorageListRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }
    pub fn with_filter_regexp(mut self, filter_regexp: String) -> Self {
        self.filter_regexp = Some(filter_regexp);
        self
    }
    pub fn with_filter_regexp_list(mut self, filter_regexp_list: Vec<String>) -> Self {
        self.filter_regexp_list = Some(filter_regexp_list);
        self
    }
    pub fn with_page_offset(mut self, page_offset: isize) -> Self {
        self.page_offset = Some(page_offset);
        self
    }
    pub fn with_page_size(mut self, page_size: isize) -> Self {
        self.page_size = Some(page_size);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiStorageListResponse {
    #[serde(rename = "Files")]
    pub files: Vec<FileInfo>,
    #[serde(rename = "Total")]
    pub total: isize,
    #[serde(rename = "NextMarker")]
    pub next_marker: isize,
}

impl HttpBuilder for ApiStorageListRequest {
    type Response = BaseResponse<ApiStorageListResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || {
                let mut query_params = HashMap::new();
                if let Some(ref path) = self.path {
                    query_params.insert("Path".to_string(), path.clone());
                }
                if let Some(ref filter_regexp) = self.filter_regexp {
                    query_params.insert("FilterRegexp".to_string(), filter_regexp.clone());
                }
                if let Some(page_offset) = self.page_offset {
                    query_params.insert("PageOffset".to_string(), page_offset.to_string());
                }
                if let Some(page_size) = self.page_size {
                    query_params.insert("PageSize".to_string(), page_size.to_string());
                }
                BaseRequest {
                    method: Method::GET,
                    uri: "/api/storage/lsWithPage".to_string(),
                    queries: Some(query_params),
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
    async fn test_api_storage_list() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let user_id = config.user_id.clone();
        let mut client = OpenApiClient::new(config).with_endpoint_type(EndpointType::Cloud);

        let http_fn = ApiStorageListRequest::new()
            .with_path(format!("/{}", user_id))
            .with_page_offset(0)
            .with_page_size(10)
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
