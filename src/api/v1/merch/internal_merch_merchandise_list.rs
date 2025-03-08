use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use crate::model::merch::{ChargeType, Merchandise, PublishState};
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalMerchMerchandiseListRequest {
    #[serde(rename = "OutResourceId")]
    pub out_resource_id: Option<String>,
    #[serde(rename = "YSProduct")]
    pub ys_product: Option<String>,
    #[serde(rename = "ChargeType")]
    pub charge_type: Option<ChargeType>,
    #[serde(rename = "PublishState")]
    pub publish_state: Option<PublishState>,
    #[serde(rename = "PageOffset")]
    pub page_offset: Option<isize>,
    #[serde(rename = "PageSize")]
    pub page_size: Option<isize>,
}

impl InternalMerchMerchandiseListRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_out_resource_id(mut self, out_resource_id: String) -> Self {
        self.out_resource_id = Some(out_resource_id);
        self
    }
    pub fn with_ys_product(mut self, ys_product: String) -> Self {
        self.ys_product = Some(ys_product);
        self
    }
    pub fn with_charge_type(mut self, charge_type: ChargeType) -> Self {
        self.charge_type = Some(charge_type);
        self
    }
    pub fn with_publish_state(mut self, publish_state: PublishState) -> Self {
        self.publish_state = Some(publish_state);
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
pub struct InternalMerchMerchandiseListResponse {
    #[serde(rename = "Merchandises")]
    pub merchandises: Vec<Merchandise>,
    #[serde(rename = "Offset")]
    pub offset: isize,
    #[serde(rename = "Size")]
    pub size: isize,
    #[serde(rename = "Total")]
    pub total: isize,
    #[serde(rename = "NextMarker")]
    pub next_marker: isize,
}

impl HttpBuilder for InternalMerchMerchandiseListRequest {
    type Response = BaseResponse<InternalMerchMerchandiseListResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || {
                let mut queries = HashMap::new();
                if let Some(ref out_resource_id) = self.out_resource_id {
                    queries.insert("OutResourceId".to_string(), out_resource_id.to_string());
                }
                if let Some(ref ys_product) = self.ys_product {
                    queries.insert("YSProduct".to_string(), ys_product.to_string());
                }
                if let Some(ref charge_type) = self.charge_type {
                    queries.insert("ChargeType".to_string(), charge_type.to_string());
                }
                if let Some(ref publish_state) = self.publish_state {
                    queries.insert("PublishState".to_string(), publish_state.to_string());
                }
                if let Some(ref page_offset) = self.page_offset {
                    queries.insert("PageOffset".to_string(), page_offset.to_string());
                }
                if let Some(ref page_size) = self.page_size {
                    queries.insert("PageSize".to_string(), page_size.to_string());
                }
                BaseRequest {
                    method: Method::GET,
                    uri: "/internal/merchandises".to_string(),
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
    async fn test_internal_merch_merchandise_list() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let mut client = OpenApiClient::new(config);

        let http_fn = InternalMerchMerchandiseListRequest::new().builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
