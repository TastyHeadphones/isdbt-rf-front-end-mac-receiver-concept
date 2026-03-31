use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use isdbt_shared::protocol::{ReplayRequest, TuneRequest};
use reqwest::Client;

#[derive(Debug, Parser)]
#[command(author, version, about = "ISDB-T reference platform CLI")]
struct Cli {
    #[arg(long, default_value = "http://127.0.0.1:8088")]
    endpoint: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Discover,
    Tune {
        #[arg(long)]
        device: String,
        #[arg(long)]
        frequency_hz: u32,
        #[arg(long)]
        bandwidth_mhz: u8,
    },
    Metrics,
    Replay {
        #[arg(long)]
        file: String,
    },
    Health,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = Client::new();

    match cli.command {
        Commands::Discover => {
            let url = format!("{}/api/v1/devices", cli.endpoint);
            let response = client.get(url).send().await.context("request failed")?;
            print_json(response).await?;
        }
        Commands::Tune {
            device,
            frequency_hz,
            bandwidth_mhz,
        } => {
            let url = format!("{}/api/v1/tune", cli.endpoint);
            let response = client
                .post(url)
                .json(&TuneRequest {
                    device_id: device,
                    frequency_hz,
                    bandwidth_mhz,
                })
                .send()
                .await
                .context("request failed")?;
            print_json(response).await?;
        }
        Commands::Metrics => {
            let url = format!("{}/api/v1/metrics", cli.endpoint);
            let response = client.get(url).send().await.context("request failed")?;
            print_json(response).await?;
        }
        Commands::Replay { file } => {
            let url = format!("{}/api/v1/replay", cli.endpoint);
            let response = client
                .post(url)
                .json(&ReplayRequest { file })
                .send()
                .await
                .context("request failed")?;
            print_json(response).await?;
        }
        Commands::Health => {
            let url = format!("{}/health", cli.endpoint);
            let response = client.get(url).send().await.context("request failed")?;
            print_json(response).await?;
        }
    }

    Ok(())
}

async fn print_json(response: reqwest::Response) -> Result<()> {
    let status = response.status();
    let body = response
        .text()
        .await
        .context("failed to read response body")?;

    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&body) {
        println!("{}", serde_json::to_string_pretty(&value)?);
    } else {
        println!("{body}");
    }

    if !status.is_success() {
        anyhow::bail!("request failed with status {status}");
    }

    Ok(())
}
