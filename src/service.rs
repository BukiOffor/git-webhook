use super::*;
use tokio::process::Command;
//use tracing::*;

pub async fn run_worker(mut rx: tokio::sync::mpsc::Receiver<()> ) {
    tracing::info!("Worker started");
    
    while let Some(_) = rx.recv().await {
        tracing::debug!("Worker received signal");
        handle_deploy().await.unwrap_or_else(|e| {
            tracing::error!("Failed to deploy: {}", e);
        });
    }
}

async fn handle_deploy() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the latest deploy script embedded in the binary is written before running
    if let Err(e) = write_deploy_script() {
        tracing::error!("Failed to write deploy script before spawn: {}", e);
    }

    // Detach a child process that outlives this server
    let log = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/var/log/deploy.log")?;

    Command::new(DEPLOY_PATH)
        .stdout(log.try_clone()?)
        .stderr(log)
        .spawn()?;
    Ok(())
}