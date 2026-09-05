use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

//TODO: add 'separate structs' for like packages style stuff

#[derive(Debug, Serialize, Deserialize)]
pub struct VisualConfigs {
    pub font_size: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub visuals: VisualConfigs,
}

impl Default for VisualConfigs {
    fn default() -> Self {
        Self { font_size: 12.0 }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            visuals: VisualConfigs::default(),
        }
    }
}

pub fn read_config() -> Result<Config> {
    let path = Path::new("config.toml");
    if !path.exists() {
        println!("no exist");
        let mut file = File::create("config.toml")?;
        let config = Config::default();
        write!(&mut file, "{}", toml::to_string_pretty(&config)?)?;
        return Ok(config);
    }
    let content: String = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
