use std::process::{Child, Command, Stdio};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PackageInstalled {
    pub version: String,
    pub time: i32,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    tap: String,
    pub desc: String,
    pub installed: Vec<PackageInstalled>,
}

impl Package {
    pub fn is_cask(&self) -> bool {
        self.tap != "homebrew/core"
    }
}

pub fn get_installed_packages() -> Result<Vec<Package>, String> {
    let output = Command::new("brew")
        .args(["info", "--installed", "--json", "-q"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .map_err(|e| format!("error in `brew info`: {}", e))?;
    let output = output.stdout.as_slice();

    serde_json::from_slice::<Vec<Package>>(output)
        .map_err(|e| format!("error deserializing brew output: {}", e))
}

pub fn upgrade_packages() -> Result<Child, String> {
    let cmd = Command::new("brew")
        .args(["upgrade", "--greedy", "-n"])
        .env("HOMEBREW_COLOR", "true")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("error in `brew upgrade`: {}", e))?;

    Ok(cmd)
}
