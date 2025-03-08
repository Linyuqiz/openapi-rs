use crate::common::define::{
    AsyncResponseFn, BaseRequest, BaseResponse, HttpBuilder, HttpFn, RequestFn,
};
use crate::model::merch::ChargeType;
use bytes::Bytes;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalMerchMerchandisePostRequest {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ChargeType")]
    pub charge_type: Option<ChargeType>,
    #[serde(rename = "UnitPrice")]
    pub unit_price: Option<f64>,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: Option<String>,
    #[serde(rename = "Formula")]
    pub formula: Option<String>,
    #[serde(rename = "YSProduct")]
    pub ys_product: Option<String>,
    #[serde(rename = "OutResourceId")]
    pub out_resource_id: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
}

impl InternalMerchMerchandisePostRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    pub fn with_charge_type(mut self, charge_type: ChargeType) -> Self {
        self.charge_type = Some(charge_type);
        self
    }
    pub fn with_unit_price(mut self, unit_price: f64) -> Self {
        self.unit_price = Some(unit_price);
        self
    }
    pub fn with_quantity_unit(mut self, quantity_unit: String) -> Self {
        self.quantity_unit = Some(quantity_unit);
        self
    }
    pub fn with_formula(mut self, formula: String) -> Self {
        self.formula = Some(formula);
        self
    }
    pub fn with_ys_product(mut self, ys_product: String) -> Self {
        self.ys_product = Some(ys_product);
        self
    }
    pub fn with_out_resource_id(mut self, out_resource_id: String) -> Self {
        self.out_resource_id = Some(out_resource_id);
        self
    }
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalMerchMerchandisePostResponse {
    #[serde(rename = "Id")]
    pub id: String,
}

impl HttpBuilder for InternalMerchMerchandisePostRequest {
    type Response = BaseResponse<InternalMerchMerchandisePostResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(move || BaseRequest {
                method: Method::POST,
                uri: "/internal/merchandises".to_string(),
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
    async fn test_internal_merch_merchandise_post() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv()?;
        let config = OpenApiConfig::new().load_from_env()?;
        let mut client = OpenApiClient::new(config);

        let http_fn = InternalMerchMerchandisePostRequest::new().builder();
        let response = client.send(http_fn).await?;
        info!("response: {:#?}", response);

        Ok(())
    }
}
