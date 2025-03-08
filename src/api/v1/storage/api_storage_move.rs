use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use bytes::Bytes;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiStorageMoveRequest {
    #[serde(rename = "Src")]
    pub src_path: Option<String>,
    #[serde(rename = "Dest")]
    pub dest_path: Option<String>,
}

impl ApiStorageMoveRequest {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn with_src_path(mut self, src_path: String) -> Self {
        self.src_path = Some(src_path);
        self
    }
    pub fn with_dest_path(mut self, dest_path: String) -> Self {
        self.dest_path = Some(dest_path);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiStorageMoveResponse {}

impl HttpBuilder for ApiStorageMoveRequest {
    type Response = BaseResponse<ApiStorageMoveResponse>;

    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || {
                let body_content = serde_json::to_vec(&self).unwrap();
                BaseRequest {
                    method: Method::POST,
                    uri: "/api/storage/mv".to_string(),
                    content_type: Some("application/json".to_string()),
                    body: Bytes::from(body_content),
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
    async fn test_api_storage_move() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let user_id = config.user_id.clone();
        let mut client = OpenApiClient::new(config).with_endpoint_type(EndpointType::Cloud);

        let http_fn = ApiStorageMoveRequest::new()
            .with_src_path(format!("/{}/runner.py", user_id))
            .with_dest_path(format!("/{}/tmp/runner.py", user_id))
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
