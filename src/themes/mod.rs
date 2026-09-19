use std::path::PathBuf;

use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio_stream::StreamExt;
use tokio_tar::Archive;

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    manifest: ThemeManifest,
    // todo: maybe not load this into ram until later
    css: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct ThemeManifest {
    pub id: String,
    pub name: String,
}

impl Theme {
    pub fn id(&self) -> &str {
        &self.manifest.id
    }

    pub fn name(&self) -> &str {
        &self.manifest.name
    }

    pub fn css(&self) -> &str {
        &self.css
    }
}

pub async fn parse_theme(path: PathBuf) -> anyhow::Result<Theme> {
    let file = File::open(path).await?;
    let mut archive = Archive::new(file);

    let mut entries = archive.entries()?;

    let mut manifest = None;
    let mut css = None;
    while let Some(entry) = entries.next().await {
        let mut entry = entry?;

        let path = entry.path()?;
        let Some(name) = path.file_stem() else {
            continue;
        };
        let Some(ext) = path.extension() else {
            continue;
        };
        let name = name.to_string_lossy();
        let ext = ext.to_string_lossy();

        if name == "theme" && ext == "json" {
            let mut contents = Vec::new();
            entry.read_to_end(&mut contents).await?;
            manifest = Some(serde_json::from_slice(&contents)?);
        } else if name == "theme" && ext == "css" {
            let mut contents = String::new();
            entry.read_to_string(&mut contents).await?;
            css = Some(contents);
        }
    }
    let Some(manifest) = manifest else {
        anyhow::bail!("No theme.json file found for theme")
    };
    let Some(css) = css else {
        anyhow::bail!("No theme.css file found for theme")
    };

    Ok(Theme { manifest, css })
}

