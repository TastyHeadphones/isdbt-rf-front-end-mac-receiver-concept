use anyhow::{Context, Result};
use tokio::fs;

#[derive(Debug, Clone)]
pub struct ReplayStats {
    pub packets_ingested: u64,
    pub bytes_ingested: u64,
}

pub async fn ingest_ts_file(path: &str) -> Result<ReplayStats> {
    let data = fs::read(path)
        .await
        .with_context(|| format!("failed to read TS file: {path}"))?;

    let bytes = data.len() as u64;
    let packets = bytes / 188;

    Ok(ReplayStats {
        packets_ingested: packets,
        bytes_ingested: bytes,
    })
}
