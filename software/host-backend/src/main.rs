mod app_state;
mod transport;

use std::path::PathBuf;

use anyhow::{Context, Result};
use app_state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use clap::Parser;
use isdbt_shared::config::{LoggingConfig, PlatformConfig, RuntimeMode};
use isdbt_shared::protocol::{
    HealthResponse, ReplayRequest, ReplayResponse, TuneRequest, TuneResponse,
};
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[command(author, version, about = "ISDB-T reference host backend")]
struct Args {
    #[arg(long, default_value = "shared/config/default.toml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config = PlatformConfig::load_from_file(&args.config)
        .with_context(|| format!("loading config from {}", args.config.display()))?;

    init_logging(&config.logging);

    let state = AppState::new(config.clone());
    state.initialize().await;

    if config.lab.autostart_replay {
        if let Some(path) = &config.lab.default_replay_file {
            match transport::ingest_ts_file(path).await {
                Ok(stats) => {
                    let _ = state
                        .record_replay(path.clone(), stats.packets_ingested, stats.bytes_ingested)
                        .await;
                    info!(file = %path, packets = stats.packets_ingested, "autostart replay loaded");
                }
                Err(err) => {
                    warn!(error = %err, file = %path, "autostart replay failed");
                }
            }
        }
    }

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/devices", get(list_devices))
        .route("/api/v1/tune", post(tune_device))
        .route("/api/v1/metrics", get(metrics))
        .route("/api/v1/replay", post(replay))
        .with_state(state.clone());

    let bind_addr = config.bind_addr();
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .with_context(|| format!("binding HTTP listener to {bind_addr}"))?;

    info!(bind_addr = %bind_addr, mode = %mode_label(&config.mode), "host-backend started");

    axum::serve(listener, app).await.context("serving HTTP API")?;
    Ok(())
}

fn init_logging(logging: &LoggingConfig) {
    let filter = EnvFilter::try_new(logging.level.clone()).unwrap_or_else(|_| EnvFilter::new("info"));

    if logging.format.eq_ignore_ascii_case("json") {
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .json()
            .with_current_span(false)
            .with_span_list(false)
            .init();
    } else {
        tracing_subscriber::fmt().with_env_filter(filter).init();
    }
}

fn mode_label(mode: &RuntimeMode) -> &'static str {
    match mode {
        RuntimeMode::LabDemo => "lab_demo",
        RuntimeMode::RealRfIntegration => "real_rf_integration",
    }
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        mode: mode_label(&state.config().mode).to_string(),
    })
}

async fn list_devices(State(state): State<AppState>) -> Json<Vec<isdbt_shared::protocol::DeviceSummary>> {
    Json(state.list_devices().await)
}

async fn tune_device(
    State(state): State<AppState>,
    Json(req): Json<TuneRequest>,
) -> Result<Json<TuneResponse>, (StatusCode, String)> {
    let response = state.tune(req).await;
    if response.accepted {
        Ok(Json(response))
    } else {
        Err((StatusCode::NOT_FOUND, response.message))
    }
}

async fn metrics(State(state): State<AppState>) -> Json<isdbt_shared::protocol::MetricsResponse> {
    Json(state.metrics().await)
}

async fn replay(
    State(state): State<AppState>,
    Json(req): Json<ReplayRequest>,
) -> Result<Json<ReplayResponse>, (StatusCode, String)> {
    let stats = transport::ingest_ts_file(&req.file).await.map_err(|err| {
        error!(error = %err, file = %req.file, "replay ingestion failed");
        (StatusCode::BAD_REQUEST, err.to_string())
    })?;

    state
        .record_replay(req.file.clone(), stats.packets_ingested, stats.bytes_ingested)
        .await;

    Ok(Json(ReplayResponse {
        packets_ingested: stats.packets_ingested,
        bytes_ingested: stats.bytes_ingested,
        file: req.file,
    }))
}
