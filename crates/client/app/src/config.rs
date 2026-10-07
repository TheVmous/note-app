use anyhow::Result;
use serde::{Deserialize, Serialize};
#[cfg(not(target_family = "wasm"))]
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

// TODO: add 'separate structs' for like packages style stuff

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub visuals: VisualConfigs,
    pub syst: SystConfigs,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystConfigs {
    pub default_ext: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VisualConfigs {
    pub font_size: f32,
    pub tab_len: i32,
    #[serde(default)]
    pub theme: Option<String>,
}

/* end config structs */

impl Default for VisualConfigs {
    fn default() -> Self {
        Self {
            font_size: 12.0,
            tab_len: 4,
            theme: None,
        }
    }
}

impl Default for SystConfigs {
    fn default() -> Self {
        Self {
            default_ext: String::from("md"),
        }
    }
}

#[cfg(target_family = "wasm")]
pub fn read_config() -> Result<Config> {
    Ok(Config::default())
}

#[cfg(not(target_family = "wasm"))]
fn config_path() -> PathBuf {
    let local = Path::new("config.toml");
    if local.exists() {
        return local.to_path_buf();
    }
    crate::fs::config_dir().join("config.toml")
}

#[cfg(not(target_family = "wasm"))]
pub fn read_config() -> Result<Config> {
    let path = config_path();
    if !path.exists() {
        let mut file = File::create(&path)?;
        let config = Config::default();
        write!(&mut file, "{}", toml::to_string_pretty(&config)?)?;
        return Ok(config);
    }
    let content: String = std::fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
