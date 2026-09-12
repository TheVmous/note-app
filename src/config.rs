use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::print;

// TODO: add 'separate structs' for like packages style stuff




#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub visuals: VisualConfigs,
    pub syst: SystConfigs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystConfigs {
    pub default_ext: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisualConfigs {
    pub font_size: f32,
    pub tab_len: i32,
}

/* end config structs */

impl Default for VisualConfigs {
    fn default() -> Self {
        Self {
            font_size: 12.0,
            tab_len: 4,
        }
    }
}

impl Default for SystConfigs {
    fn default() -> Self {
        Self {
            default_ext: String::from(".md"),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            visuals: VisualConfigs::default(),
            syst: SystConfigs::default(),
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
    print!("{}", config.syst.default_ext);
    Ok(config)
}
