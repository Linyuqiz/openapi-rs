use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalRdpGoCleanRequest {
    #[serde(rename = "PrivateIP")]
    pub private_ip: Option<String>,
    #[serde(rename = "x-ys-request-id")]
    pub request_id: Option<String>,
}

impl InternalRdpGoCleanRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_private_ip(mut self, private_ip: String) -> Self {
        self.private_ip = Some(private_ip);
        self
    }
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalRdpGoCleanResponse {}

impl HttpBuilder for InternalRdpGoCleanRequest {
    type Response = BaseResponse<InternalRdpGoCleanResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || {
                let mut headers = HeaderMap::new();
                if let Some(ref request_id) = self.request_id {
                    headers.insert(
                        HeaderName::from_bytes("x-ys-request-id".as_bytes()).unwrap(),
                        HeaderValue::from_str(request_id).unwrap(),
                    );
                }
                let mut queries = HashMap::new();
                if let Some(ref private_ip) = self.private_ip {
                    queries.insert("PrivateIP".to_string(), private_ip.clone());
                }
                BaseRequest {
                    method: Method::POST,
                    uri: "/internal/clean".to_string(),
                    headers,
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
    use crate::common::config::OpenApiConfig;
    use tracing::info;

    #[tokio::test]
    async fn test_job_list() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let mut client = OpenApiClient::new(config);

        let http_fn = InternalRdpGoCleanRequest::new()
            .with_private_ip("123".to_string())
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
