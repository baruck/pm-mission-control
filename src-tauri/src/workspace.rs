use anyhow::{anyhow, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMarker {
    pub workspaceVersion: u8,
    pub createdAt: String,
    pub deviceId: String,
}

pub fn initialize_workspace(root: &Path, device_id: &str) -> Result<PathBuf> {
    if !root.exists() {
        return Err(anyhow!("workspace folder does not exist"));
    }

    let dirs = ["journal", "tasks", "artifacts", "links", "settings"];
    for dir in dirs {
        fs::create_dir_all(root.join(dir))?;
    }

    let marker = WorkspaceMarker {
        workspaceVersion: 1,
        createdAt: Utc::now().to_rfc3339(),
        deviceId: device_id.to_string(),
    };

    let marker_path = root.join(".missioncontrol.json");
    fs::write(&marker_path, serde_json::to_vec_pretty(&marker)?)?;
    Ok(marker_path)
}
