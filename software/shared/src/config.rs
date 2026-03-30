use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeMode {
    LabDemo,
    RealRfIntegration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    pub bind: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabConfig {
    pub simulate_devices: bool,
    pub autostart_replay: bool,
    pub default_replay_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfig {
    pub mode: RuntimeMode,
    pub http: HttpConfig,
    pub lab: LabConfig,
    pub logging: LoggingConfig,
}

impl PlatformConfig {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path_ref = path.as_ref();
        let raw = fs::read_to_string(path_ref)
            .with_context(|| format!("failed to read config file: {}", path_ref.display()))?;

        toml::from_str(&raw)
            .with_context(|| format!("failed to parse TOML config: {}", path_ref.display()))
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.http.bind, self.http.port)
    }
}
