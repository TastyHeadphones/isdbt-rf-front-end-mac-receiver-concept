use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSummary {
    pub id: String,
    pub mode: String,
    pub locked: bool,
    pub snr_db: f32,
    pub ber: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuneRequest {
    pub device_id: String,
    pub frequency_hz: u32,
    pub bandwidth_mhz: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuneResponse {
    pub device_id: String,
    pub accepted: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayRequest {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayResponse {
    pub packets_ingested: u64,
    pub bytes_ingested: u64,
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsResponse {
    pub packets_total: u64,
    pub bytes_total: u64,
    pub last_replay_file: Option<String>,
    pub last_update_unix_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub mode: String,
}
