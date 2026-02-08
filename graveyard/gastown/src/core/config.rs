use anyhow::{Context, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub rigs: HashMap<String, String>, // Name -> Repo URL
    #[serde(default)]
    pub convoys: HashMap<String, Convoy>,
    #[serde(default)]
    pub agents: HashMap<String, Agent>,
    #[serde(default)]
    pub tasks: HashMap<String, Task>, // Bead ID -> Task
    pub current_convoy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Convoy {
    pub id: String,
    pub name: String,
    pub tasks: Vec<String>, // Bead IDs
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub status: String,
    pub current_task: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub status: String,
}

impl Config {
    pub fn config_path() -> Result<PathBuf> {
        let mut path = home_dir().context("Could not find home directory")?;
        path.push(".gastown");
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        path.push("config.json");
        Ok(path)
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Config::default());
        }
        let content = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}
