use crate::err;
use anyhow::{anyhow, Context, Result};
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub prefix_path: PathBuf,
    pub default_hooks: Vec<String>,
    pub env: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        let mut prefix_path = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("~/"));
        prefix_path.push(".wine");

        Self {
            prefix_path,
            default_hooks: Vec::new(),
            env: HashMap::new(),
        }
    }

    pub fn load() -> Result<Self> {
        let path = Self::validated_path()?;
        let contents = fs::read_to_string(&path)
            .with_context(|| anyhow!("failed to read config file at {}", path.display()))?;

        toml::from_str(&contents)
            .with_context(|| anyhow!("failed to decode config file at {}", path.display()))
    }

    pub fn load_or_create() -> Result<Self> {
        match Self::load() {
            Ok(config) => Ok(config),
            Err(err) if err::is_file_nonexistant(&err) => {
                let config = Self::new();
                config.save()?;
                Ok(config)
            }
            Err(err) => Err(err),
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::validated_path()?;
        let serialized = toml::to_string_pretty(self).context("failed to encode config file")?;

        fs::write(&path, serialized)
            .with_context(|| anyhow!("failed to write config file to {}", path.display()))
    }

    pub fn validated_path() -> Result<PathBuf> {
        static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
            let mut dir = dirs_next::config_dir().unwrap_or_else(|| PathBuf::from("~/.config/"));
            dir.push(env!("CARGO_PKG_NAME"));
            dir
        });

        if !CONFIG_PATH.exists() {
            fs::create_dir_all(&*CONFIG_PATH).context("failed to create config file dir")?;
        }

        let mut path = CONFIG_PATH.clone();
        path.push("config.toml");
        Ok(path)
    }
}
