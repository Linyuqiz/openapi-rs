use bytes::Bytes;
use openapi_common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RemoveRequest {
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "IgnoreNotExist")]
    pub ignore_not_exist: Option<bool>,
}

impl RemoveRequest {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }
    pub fn with_ignore_not_exist(mut self, ignore_not_exist: bool) -> Self {
        self.ignore_not_exist = Some(ignore_not_exist);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RemoveResponse {}

impl HttpBuilder for RemoveRequest {
    type Response = BaseResponse<RemoveResponse>;

    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || {
                let body_content = serde_json::to_vec(&self).unwrap();
                BaseRequest {
                    method: Method::POST,
                    uri: "/api/storage/rm".to_string(),
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
    use openapi_common::client::OpenApiClient;
    use openapi_common::config::{EndpointType, OpenApiConfig};
    use tracing::info;

    #[tokio::test]
    async fn test_move() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let user_id = config.user_id.clone();
        let mut client = OpenApiClient::new(config).with_endpoint_type(EndpointType::Cloud);

        let http_fn = RemoveRequest::new()
            .with_path(format!("/{}/runner.py", user_id))
            .with_ignore_not_exist(true)
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
