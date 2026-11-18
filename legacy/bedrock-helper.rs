//! One application owns login, licensed executable preparation, and Xbox IPC.
use std::{path::{Path, PathBuf}, sync::Arc, process::ExitCode};
use xodus::tokens::{TokenManager, store::TokenStoreError};
#[path = "../src/webview.rs"]
mod webview;
#[path = "../src/commands/login.rs"]
mod login;
#[path = "bedrock_helper_parts/prepare_owned_game.rs"]
mod prepare_owned_game;

fn require_proprietary_acceptance() -> Result<(), &'static str> {
    let root = match std::env::var_os("BEDROCK_HOME") {
        Some(path) => PathBuf::from(path),
        None => std::env::current_dir().map_err(|_| "The project root is unavailable.")?,
    };
    let eula = root.join("EULA.md");
    let metadata = std::fs::symlink_metadata(&eula)
        .map_err(|_| "Review EULA.md and enable proprietary-component acceptance before use.")?;
    if !metadata.file_type().is_file() {
        return Err("EULA.md must be a regular file.");
    }
    let text = std::fs::read_to_string(eula).map_err(|_| "EULA.md is unreadable.")?;
    let versions: Vec<_> = text.lines().map(str::trim)
        .filter(|line| line.starts_with("EULA_VERSION")).collect();
    let acceptances: Vec<_> = text.lines().map(str::trim)
        .filter(|line| line.starts_with("PROPRIETARY_COMPONENTS_ACCEPTED")).collect();
    if versions.len() != 1 || versions[0] != "EULA_VERSION = 1"
        || acceptances.len() != 1 || acceptances[0] != "PROPRIETARY_COMPONENTS_ACCEPTED = true" {
        return Err("Proprietary component use is disabled; review EULA.md and enable its current acceptance setting.");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    require_proprietary_acceptance()?;
    let directory = std::env::args().nth(1).ok_or("game directory required")?;
    if Path::new("/tmp/xodus.sock").exists() {
        return Err("An Xbox service socket already exists; stop its owner before starting another helper.".into());
    }
    xodus::secrets::init_named_secrets("Minecraft Bedrock Mac")?;
    let tokens = Arc::new(TokenManager::with_keychain_and_memory());
    let client = reqwest::Client::builder().user_agent("bedrock-mac-helper/0.1").build()?;
    xodus::tokens::device::ensure_device_credentials(&client, &tokens).await;
    let needs_login = match tokens.get_user() {
        Ok(_) => false,
        Err(TokenStoreError::NotFound) => true,
        Err(error) => return Err(error.into()),
    };
    if needs_login {
        eprintln!("Complete Microsoft sign-in in the login window.");
        if login::run(&client, &tokens).await != ExitCode::SUCCESS {
            return Err("Microsoft sign-in was not completed".into());
        }
    }
    tokens.get_user_sts_token()?;
    eprintln!("Microsoft sign-in available. Preparing the owned Windows game.");
    prepare_owned_game::prepare(Path::new(&directory), &client, &tokens).await?;
    eprintln!("Starting Xbox account service.");
    xodus_service::run(tokens).await;
    Ok(())
}
