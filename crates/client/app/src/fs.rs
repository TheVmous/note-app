#![cfg(not(target_family = "wasm"))]

use std::{env, path::PathBuf};

pub fn config_dir() -> PathBuf {
    if let Ok(dir) = env::var("EDITOR_CONFIG_FOLDER") {
        return dir.into();
    }

    let dir = dirs::config_dir()
        .expect("No config directory could be inferred for this operating system. Set EDITOR_CONFIG_FOLDER to override it.")
        .join("editor");
    std::fs::create_dir_all(&dir)
        .unwrap_or_else(|_| panic!("Could not create config directory {}", dir.display()));
    dir
}

pub fn themes_dir() -> PathBuf {
    let dir = config_dir().join("themes");
    std::fs::create_dir_all(&dir)
        .unwrap_or_else(|_| panic!("Could not create themes directory {}", dir.display()));
    dir
}
