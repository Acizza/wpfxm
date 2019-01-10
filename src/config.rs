use crate::error::ConfigError;
use crate::util::dir;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub type Filename = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_base_directory")]
    pub base_directory: PathBuf,
    #[serde(default)]
    pub setup_hooks: Vec<Filename>,
    #[serde(default)]
    pub global_env_vars: HashMap<String, String>,
}

impl Config {
    pub const DEFAULT_FILENAME: &'static str = "config.toml";

    pub fn load<P>(path: Option<P>) -> Result<Config, ConfigError>
    where
        P: Into<PathBuf>,
    {
        let path = match path {
            Some(path) => path.into(),
            None => Config::get_path()?,
        };

        let contents = fs::read_to_string(path).map_err(ConfigError::FailedToRead)?;
        let config = toml::from_str(&contents).map_err(ConfigError::FailedToParse)?;

        Ok(config)
    }

    pub fn save(&self) -> Result<PathBuf, ConfigError> {
        let path = Config::get_path()?;
        let toml = toml::to_string(self).map_err(ConfigError::FailedToSerialize)?;

        fs::write(&path, toml)
            .map_err(|err| ConfigError::FailedToWrite(err, path.to_string_lossy().to_string()))?;

        Ok(path)
    }

    pub fn get_path() -> Result<PathBuf, ConfigError> {
        let mut path = dir::get_config_dir().ok_or(ConfigError::FailedToGetConfigDir)?;
        path.push(Config::DEFAULT_FILENAME);
        Ok(path)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            base_directory: default_base_directory(),
            setup_hooks: Vec::new(),
            global_env_vars: HashMap::new(),
        }
    }
}

fn default_base_directory() -> PathBuf {
    dirs::home_dir()
        .map(|hd| hd.join("wine"))
        .unwrap_or_else(|| PathBuf::from("~/wine"))
}
