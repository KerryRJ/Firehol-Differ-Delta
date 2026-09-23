use anyhow::{Context, Result};
use firehol::{load_config, run_scheduler};
use std::{fs, path::Path};
use tokio_util::sync::CancellationToken;

fn init_logging() -> Result<()> {
    let log_dir = Path::new("/var/log/firehol-differ-delta");
    fs::create_dir_all(log_dir).context("Failed to create /var/log/firehol-differ-delta")?;
    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_dir.join("firehol.log"))
        .context("Failed to open /var/log/firehol-differ-delta/firehol.log")?;
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .target(env_logger::Target::Pipe(Box::new(file)))
        .init();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logging()?;
    let config = load_config(Path::new(".")).await?;
    let data_dir = config.path.clone();
    let cancellation = CancellationToken::new();
    let scheduler = tokio::spawn(run_scheduler(data_dir, config, cancellation.clone()));
    tokio::select! {
        result = scheduler => result.context("Scheduler task failed")??,
        result = tokio::signal::ctrl_c() => {
            result.context("Failed to listen for shutdown signal")?;
            cancellation.cancel();
        }
    }
    Ok(())
}
