use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct JobInfo {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "JobState")]
    pub job_state: String,
    #[serde(rename = "FileSyncState")]
    pub file_sync_state: String,
    #[serde(rename = "StateReason")]
    pub state_reason: String,
    #[serde(rename = "AllocResource")]
    pub alloc_resource: AllocResource,
    #[serde(rename = "AllocType")]
    pub alloc_type: String,
    #[serde(rename = "ExecHostNum")]
    pub exec_host_num: i32,
    #[serde(rename = "Zone")]
    pub zone: String,
    #[serde(rename = "Workdir")]
    pub workdir: String,
    #[serde(rename = "OutputDir")]
    pub output_dir: String,
    #[serde(rename = "NoNeededPaths")]
    pub no_needed_paths: String,
    #[serde(rename = "NeededPaths")]
    pub needed_paths: String,
    #[serde(rename = "Parameters")]
    pub parameters: String,
    #[serde(rename = "NoRound")]
    pub no_round: bool,

    #[serde(rename = "PendingTime")]
    pub pending_time: String,
    #[serde(rename = "RunningTime")]
    pub running_time: String,
    #[serde(rename = "TerminatingTime")]
    pub terminating_time: String,
    #[serde(rename = "SuspendingTime")]
    pub suspending_time: String,
    #[serde(rename = "SuspendedTime")]
    pub suspended_time: String,
    #[serde(rename = "EndTime")]
    pub end_time: String,
    #[serde(rename = "CreateTime")]
    pub create_time: String,
    #[serde(rename = "UpdateTime")]
    pub update_time: String,

    #[serde(rename = "FileReadyTime")]
    pub file_ready_time: String,
    #[serde(rename = "TransmittingTime")]
    pub transmitting_time: String,
    #[serde(rename = "TransmittedTime")]
    pub transmitted_time: String,

    #[serde(rename = "DownloadProgress")]
    pub download_progress: Progress,
    #[serde(rename = "UploadProgress")]
    pub upload_progress: Progress,

    #[serde(rename = "ExecutionDuration")]
    pub execution_duration: i32,
    #[serde(rename = "ExitCode")]
    pub exit_code: String,
    #[serde(rename = "IsSystemFailed")]
    pub is_system_failed: bool,
    #[serde(rename = "StdoutPath")]
    pub stdout_path: String,
    #[serde(rename = "StderrPath")]
    pub stderr_path: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AdminJobInfo {
    #[serde(flatten)]
    pub job_info: JobInfo,
    #[serde(rename = "Queue")]
    pub queue: String,
    #[serde(rename = "Priority")]
    pub priority: i32,
    #[serde(rename = "OriginJobID")]
    pub origin_job_id: String,
    #[serde(rename = "ExecHosts")]
    pub exec_hosts: String,
    #[serde(rename = "SubmitTime")]
    pub submit_time: String,
    #[serde(rename = "UserID")]
    pub user_id: String,
    #[serde(rename = "HPCJobID")]
    pub hpc_job_id: String,
    #[serde(rename = "IsDeleted")]
    pub is_deleted: bool,
}

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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct JobCpuUsage {
    #[serde(rename = "JobID")]
    pub job_id: String,
    #[serde(rename = "AverageCpuUsage")]
    pub average_cpu_usage: f64,
    #[serde(rename = "NodeUsages")]
    pub node_usages: HashMap<String, f64>,
}
