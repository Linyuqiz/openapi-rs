use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use bytes::Bytes;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalMerchSpecialPricePostRequest {
    #[serde(rename = "MerchandiseId")]
    pub merchandise_id: Option<String>,
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,
    #[serde(rename = "UnitPrice")]
    pub unit_price: Option<f64>,
}

impl InternalMerchSpecialPricePostRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_merchandise_id(mut self, merchandise_id: String) -> Self {
        self.merchandise_id = Some(merchandise_id);
        self
    }
    pub fn with_account_id(mut self, account_id: String) -> Self {
        self.account_id = Some(account_id);
        self
    }
    pub fn with_unit_price(mut self, unit_price: f64) -> Self {
        self.unit_price = Some(unit_price);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalMerchSpecialPricePostResponse {}

impl HttpBuilder for InternalMerchSpecialPricePostRequest {
    type Response = BaseResponse<InternalMerchSpecialPricePostResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || BaseRequest {
                method: Method::POST,
                uri: "/internal/specialprices".to_string(),
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
    use crate::common::config::OpenApiConfig;
    use tracing::info;

    #[tokio::test]
    async fn test_internal_merch_special_price_post() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let mut client = OpenApiClient::new(config);

        let http_fn = InternalMerchSpecialPricePostRequest::new()
            .with_merchandise_id("123".to_string())
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
