use crate::err::{self, Result};
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use snafu::ResultExt;
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
        let mut prefix_path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~/"));
        prefix_path.push(".wine");

        Self {
            prefix_path,
            default_hooks: Vec::new(),
            env: HashMap::new(),
        }
    }

    pub fn load() -> Result<Self> {
        let path = Self::validated_path()?;
        let contents = fs::read_to_string(&path).context(err::FileIO { path: &path })?;
        toml::from_str(&contents).context(err::TomlDecode { path })
    }

    pub fn load_or_create() -> Result<Self> {
        match Self::load() {
            Ok(config) => Ok(config),
            Err(err) if err.is_file_nonexistant() => {
                let config = Self::new();
                config.save()?;
                Ok(config)
            }
            Err(err) => Err(err),
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::validated_path()?;
        let serialized = toml::to_string_pretty(self).context(err::TomlEncode { path: &path })?;
        fs::write(&path, serialized).context(err::FileIO { path })
    }

    pub fn validated_path() -> Result<PathBuf> {
        static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
            let mut dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config/"));
            dir.push(env!("CARGO_PKG_NAME"));
            dir
        });

        if !CONFIG_PATH.exists() {
            fs::create_dir_all(&*CONFIG_PATH).context(err::FileIO {
                path: CONFIG_PATH.clone(),
            })?;
        }

        let mut path = CONFIG_PATH.clone();
        path.push("config.toml");
        Ok(path)
    }
}
