use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SyncTask {
    #[serde(rename = "JobId")]
    pub job_id: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "DownloadFinished")]
    pub download_finished: bool,
    #[serde(rename = "DownloadFileSizeCurrent")]
    pub download_file_size_current: isize,
    #[serde(rename = "DownloadFileSizeTotal")]
    pub download_file_size_total: isize,
    #[serde(rename = "DownloadFinishedTime")]
    pub download_finished_time: String,
}
