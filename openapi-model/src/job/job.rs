use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AllocResource {
    #[serde(rename = "Cores")]
    pub cores: i32,
    #[serde(rename = "Resources")]
    pub memory: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Progress {
    #[serde(rename = "TotalSize")]
    pub total_size: i32,
    #[serde(rename = "Progress")]
    pub progress: i32,
}
