use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalMerchSpecialPriceDeleteRequest {
    #[serde(rename = "MerchandiseId")]
    pub merchandise_id: Option<String>,
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,
}

impl InternalMerchSpecialPriceDeleteRequest {
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
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalMerchSpecialPriceDeleteResponse {}

impl HttpBuilder for InternalMerchSpecialPriceDeleteRequest {
    type Response = BaseResponse<InternalMerchSpecialPriceDeleteResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || {
                let mut queries = HashMap::new();
                if let Some(merchandise_id) = self.merchandise_id {
                    queries.insert("MerchandiseId".to_string(), merchandise_id);
                }
                if let Some(account_id) = self.account_id {
                    queries.insert("AccountId".to_string(), account_id);
                }
                BaseRequest {
                    method: Method::DELETE,
                    uri: "/internal/specialprices".to_string(),
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
    async fn test_internal_merch_special_price_delete() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let mut client = OpenApiClient::new(config);

        let http_fn = InternalMerchSpecialPriceDeleteRequest::new()
            .with_merchandise_id("123".to_string())
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
