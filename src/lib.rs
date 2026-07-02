pub mod handler;
pub mod service;
pub mod types;

use std::sync::Arc;

use tokio::sync::Mutex;
pub use types::*;
pub use handler::*;
pub use service::*;

use hmac::{Hmac, Mac};
use sha2::Sha256;

pub type HmacSha256 = Hmac<Sha256>;
pub type Sender = tokio::sync::mpsc::Sender<()>;
const DEPLOY_SCRIPT: &str = include_str!("../scripts/deploy.sh");
const DEPLOY_PATH: &str = "/opt/gateway/deploy.sh";

#[derive(Clone)]
pub struct AppState {
    pub sender: Sender,
    pub github_secret: Arc<String>,
    pub last_delivery: Arc<Mutex<Option<String>>>,
}


pub fn verify_signature(
    secret: &str,
    body: &[u8],
    signature: &str,
) -> bool {
    let signature = signature
        .strip_prefix("sha256=")
        .unwrap_or("");

    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(body);

    let expected = hex::encode(mac.finalize().into_bytes());

    expected.eq(signature)
}

pub fn write_deploy_script() -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let path = std::path::Path::new(DEPLOY_PATH);

    std::fs::write(path, DEPLOY_SCRIPT)?;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))?;

    tracing::info!("Deploy script written to {}", path.display());
    Ok(())
}