use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use isdbt_shared::config::PlatformConfig;
use isdbt_shared::protocol::{DeviceSummary, MetricsResponse, TuneRequest, TuneResponse};
use tokio::sync::RwLock;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    config: PlatformConfig,
    devices: Arc<RwLock<Vec<DeviceSummary>>>,
    metrics: Arc<RwLock<MetricsResponse>>,
}

impl AppState {
    pub fn new(config: PlatformConfig) -> Self {
        Self {
            config,
            devices: Arc::new(RwLock::new(Vec::new())),
            metrics: Arc::new(RwLock::new(MetricsResponse {
                packets_total: 0,
                bytes_total: 0,
                last_replay_file: None,
                last_update_unix_ms: now_unix_ms(),
            })),
        }
    }

    pub async fn initialize(&self) {
        if self.config.lab.simulate_devices {
            let mut devices = self.devices.write().await;
            devices.push(DeviceSummary {
                id: "sim-demod-001".to_string(),
                mode: "lab_demo".to_string(),
                locked: false,
                snr_db: 0.0,
                ber: 1.0,
            });
            info!(device_count = devices.len(), "initialized simulated devices");
        }
    }

    pub async fn list_devices(&self) -> Vec<DeviceSummary> {
        self.devices.read().await.clone()
    }

    pub async fn tune(&self, req: TuneRequest) -> TuneResponse {
        let mut devices = self.devices.write().await;
        if let Some(device) = devices.iter_mut().find(|d| d.id == req.device_id) {
            device.locked = true;
            device.snr_db = 30.5;
            device.ber = 0.00002;
            info!(
                device_id = %device.id,
                frequency_hz = req.frequency_hz,
                bandwidth_mhz = req.bandwidth_mhz,
                "tune request accepted"
            );
            return TuneResponse {
                device_id: device.id.clone(),
                accepted: true,
                message: "tune request accepted in lab simulation mode".to_string(),
            };
        }

        TuneResponse {
            device_id: req.device_id,
            accepted: false,
            message: "device not found".to_string(),
        }
    }

    pub async fn metrics(&self) -> MetricsResponse {
        self.metrics.read().await.clone()
    }

    pub async fn record_replay(&self, file: String, packets: u64, bytes: u64) -> MetricsResponse {
        let mut metrics = self.metrics.write().await;
        metrics.packets_total = metrics.packets_total.saturating_add(packets);
        metrics.bytes_total = metrics.bytes_total.saturating_add(bytes);
        metrics.last_replay_file = Some(file);
        metrics.last_update_unix_ms = now_unix_ms();
        metrics.clone()
    }

    pub fn config(&self) -> &PlatformConfig {
        &self.config
    }
}

fn now_unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}
