use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use crate::model::merch::SpecialPrice;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalMerchSpecialPriceListRequest {
    #[serde(rename = "MerchandiseId")]
    pub merchandise_id: Option<String>,
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,
    #[serde(rename = "PageOffset")]
    pub page_offset: Option<isize>,
    #[serde(rename = "PageSize")]
    pub page_size: Option<isize>,
}

impl InternalMerchSpecialPriceListRequest {
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
    pub fn with_page_size(mut self, page_size: isize) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn with_page_offset(mut self, page_offset: isize) -> Self {
        self.page_offset = Some(page_offset);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalMerchSpecialPriceListResponse {
    #[serde(rename = "SpecialPrices")]
    pub special_prices: Vec<SpecialPrice>,
    #[serde(rename = "Offset")]
    pub offset: isize,
    #[serde(rename = "Size")]
    pub size: isize,
    #[serde(rename = "Total")]
    pub total: isize,
    #[serde(rename = "NextMarker")]
    pub next_marker: isize,
}

impl HttpBuilder for InternalMerchSpecialPriceListRequest {
    type Response = BaseResponse<InternalMerchSpecialPriceListResponse>;
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
                if let Some(page_size) = self.page_size {
                    queries.insert("PageSize".to_string(), page_size.to_string());
                }
                if let Some(page_offset) = self.page_offset {
                    queries.insert("PageOffset".to_string(), page_offset.to_string());
                }
                BaseRequest {
                    method: Method::GET,
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
    async fn test_internal_merch_special_price_list() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let mut client = OpenApiClient::new(config);

        let http_fn = InternalMerchSpecialPriceListRequest::new()
            .with_merchandise_id("123".to_string())
            .builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
