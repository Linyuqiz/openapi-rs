use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use crate::model::merch::Merchandise;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalMerchMerchandiseGetRequest {
    #[serde(rename = "MerchandiseId")]
    pub merchandise_id: Option<String>,
}

impl InternalMerchMerchandiseGetRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_merchandise_id(mut self, merchandise_id: String) -> Self {
        self.merchandise_id = Some(merchandise_id);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalMerchMerchandiseGetResponse {
    #[serde(flatten)]
    pub merchandise: Merchandise,
}

impl HttpBuilder for InternalMerchMerchandiseGetRequest {
    type Response = BaseResponse<InternalMerchMerchandiseGetResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || BaseRequest {
                method: Method::GET,
                uri: format!("/internal/merchandises/{}", self.merchandise_id.unwrap()),
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
    use crate::common::config::OpenApiConfig;
    use tracing::info;

    #[tokio::test]
    async fn test_internal_merch_merchandise_get() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let mut client = OpenApiClient::new(config);

        let http_fn = InternalMerchMerchandiseGetRequest::new()
            .with_merchandise_id("123".to_string())
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
