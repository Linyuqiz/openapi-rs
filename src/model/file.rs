use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FileInfo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Size")]
    pub size: isize,
    #[serde(rename = "Mode")]
    pub mode: isize,
    #[serde(rename = "ModTime")]
    pub mod_time: String,
    #[serde(rename = "IsDir")]
    pub is_dir: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ChunkChecksum {
    #[serde(rename = "ChunkOffset")]
    pub chunk_offset: isize,
    #[serde(rename = "Size")]
    pub size: isize,
    #[serde(rename = "WeakChecksum")]
    pub weak_checksum: Vec<u8>,
    #[serde(rename = "StrongChecksum")]
    pub strong_checksum: Vec<u8>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Chunk {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "RoundId")]
    pub round_id: isize,
    #[serde(rename = "Priority")]
    pub priority: isize,
    #[serde(rename = "Offset")]
    pub offset: isize,
    #[serde(rename = "Length")]
    pub length: isize,
    #[serde(rename = "WeakChecksum")]
    pub weak_checksum: Vec<u8>,
    #[serde(rename = "StrongChecksum")]
    pub strong_checksum: Vec<u8>,
}
