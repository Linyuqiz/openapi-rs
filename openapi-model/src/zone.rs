use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Zone {
    #[serde(rename = "HPCEndpoint")]
    pub hpc_endpoint: String,
    #[serde(rename = "StorageEndpoint")]
    pub storage_endpoint: String,
    #[serde(rename = "CloudAppEnable")]
    pub cloud_app_enable: bool,
    #[serde(rename = "SyncRunnerEndpoint")]
    pub sync_runner_endpoint: String,
}
