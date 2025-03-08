use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Merchandise {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ChargeType")]
    pub charge_type: ChargeType,
    #[serde(rename = "UnitPrice")]
    pub unit_price: f64,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: String,
    #[serde(rename = "Formula")]
    pub formula: String,
    #[serde(rename = "YSProduct")]
    pub ys_product: String,
    #[serde(rename = "OutResourceId")]
    pub out_resource_id: String,
    #[serde(rename = "PublishState")]
    pub publish_state: PublishState,
    #[serde(rename = "Description")]
    pub description: String,
}

#[derive(Debug, Default, Display, Clone, Serialize, Deserialize)]
pub enum ChargeType {
    #[default]
    Unknown,
    PrePaid,
    PostPaid,
}

#[derive(Debug, Default, Display, Clone, Serialize, Deserialize)]
pub enum PublishState {
    #[default]
    Unknown,
    Up,
    Down,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SpecialPrice {
    #[serde(rename = "MerchandiseId")]
    pub merchandise_id: String,
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "UnitPrice")]
    pub unit_price: f64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Order {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "MerchandiseId")]
    pub merchandise_id: String,
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "Quantity")]
    pub quantity: f64,
    #[serde(rename = "Comment")]
    pub comment: String,
    #[serde(rename = "ChargeType")]
    pub charge_type: ChargeType,
    #[serde(rename = "CreateTime")]
    pub create_time: String,
}
