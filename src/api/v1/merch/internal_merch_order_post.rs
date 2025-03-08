use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use bytes::Bytes;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalMerchOrderPostRequest {
    #[serde(rename = "IdempotentId")]
    pub idempotent_id: Option<String>,
    #[serde(rename = "MerchandiseId")]
    pub merchandise_id: Option<String>,
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,
    #[serde(rename = "PayByAccountId")]
    pub pay_by_account_id: Option<String>,
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,
    #[serde(rename = "Quantity")]
    pub quantity: Option<f64>,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
}

impl InternalMerchOrderPostRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_idempotent_id(mut self, idempotent_id: String) -> Self {
        self.idempotent_id = Some(idempotent_id);
        self
    }
    pub fn with_merchandise_id(mut self, merchandise_id: String) -> Self {
        self.merchandise_id = Some(merchandise_id);
        self
    }
    pub fn with_account_id(mut self, account_id: String) -> Self {
        self.account_id = Some(account_id);
        self
    }
    pub fn with_pay_by_account_id(mut self, pay_by_account_id: String) -> Self {
        self.pay_by_account_id = Some(pay_by_account_id);
        self
    }
    pub fn with_resource_id(mut self, resource_id: String) -> Self {
        self.resource_id = Some(resource_id);
        self
    }
    pub fn with_quantity(mut self, quantity: f64) -> Self {
        self.quantity = Some(quantity);
        self
    }
    pub fn with_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalMerchOrderPostResponse {
    #[serde(rename = "Id")]
    pub id: String,
}

impl HttpBuilder for InternalMerchOrderPostRequest {
    type Response = BaseResponse<InternalMerchOrderPostResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || BaseRequest {
                method: Method::POST,
                uri: "/internal/orders".to_string(),
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
    async fn test_internal_merch_order_post() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let mut client = OpenApiClient::new(config);

        let http_fn = InternalMerchOrderPostRequest::new().builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
