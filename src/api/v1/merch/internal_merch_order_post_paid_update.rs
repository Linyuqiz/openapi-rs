use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use bytes::Bytes;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalMerchOrderPostPaidUpdateRequest {
    #[serde(rename = "OrderId")]
    pub order_id: Option<String>,
    #[serde(rename = "IdempotentId")]
    pub idempotent_id: Option<String>,
    #[serde(rename = "Quantity")]
    pub quantity: Option<f64>,
    #[serde(rename = "IsFirst")]
    pub is_first: Option<bool>,
    #[serde(rename = "IsFinished")]
    pub is_finished: Option<bool>,
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,
    #[serde(rename = "EndTime")]
    pub end_time: Option<String>,
}

impl InternalMerchOrderPostPaidUpdateRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_order_id(mut self, order_id: String) -> Self {
        self.order_id = Some(order_id);
        self
    }
    pub fn with_idempotent_id(mut self, idempotent_id: String) -> Self {
        self.idempotent_id = Some(idempotent_id);
        self
    }
    pub fn with_quantity(mut self, quantity: f64) -> Self {
        self.quantity = Some(quantity);
        self
    }
    pub fn with_is_first(mut self, is_first: bool) -> Self {
        self.is_first = Some(is_first);
        self
    }
    pub fn with_is_finished(mut self, is_finished: bool) -> Self {
        self.is_finished = Some(is_finished);
        self
    }
    pub fn with_start_time(mut self, start_time: String) -> Self {
        self.start_time = Some(start_time);
        self
    }
    pub fn with_end_time(mut self, end_time: String) -> Self {
        self.end_time = Some(end_time);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalMerchOrderPostPaidUpdateResponse {
    #[serde(rename = "Id")]
    pub id: String,
}

impl HttpBuilder for InternalMerchOrderPostPaidUpdateRequest {
    type Response = BaseResponse<InternalMerchOrderPostPaidUpdateResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || BaseRequest {
                method: Method::PATCH,
                uri: format!("/internal/orders/{}", self.order_id.as_ref().unwrap()),
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
    async fn test_internal_merch_order_post_paid_update() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let mut client = OpenApiClient::new(config);

        let http_fn = InternalMerchOrderPostPaidUpdateRequest::new().builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
