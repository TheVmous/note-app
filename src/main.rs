#![allow(non_snake_case)]

use crate::{
    buffer::Note, cli::ArgOptions, config::read_config, editor::Editor, fs::themes_dir,
    ui::open_editor,
};

pub mod buffer;
pub mod cli;
pub mod cmd;
pub mod config;
pub mod editor;
pub mod fs;
pub mod screen;
pub mod themes;
pub mod ui;

fn main() {
    let options = cli::get_options();
    let config = read_config().expect("could not open config file");

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let mut editor = Editor::new(config);
    let runtime = tokio::runtime::Runtime::new().expect("tokio runtime");
    runtime.block_on(load_editor(options, &mut editor));
    open_editor(editor);
}

pub async fn load_editor(options: ArgOptions, editor: &mut Editor) {
    let note = match options.file {
        Some(path) => Note::open(path.into()).expect("couldnt open buffer"),
        None => Note::blank(format!("untitled.{}", editor.config.syst.default_ext).into()),
    };
    editor.open_note(note, true).await;
    reload_editor(editor).await;
}

pub async fn reload_editor(editor: &mut Editor) {
    if let Err(e) = load_themes(editor).await {
        tracing::error!("Failed to load themes: {e}")
    }
    if let Some(theme) = editor.config.visuals.theme.clone()
        && let Err(e) = editor.set_theme(&theme)
    {
        tracing::error!("Error while setting theme {theme}: {e}")
    }
}

#[cfg(target_family = "wasm")]
pub async fn load_themes(editor: &mut Editor) -> anyhow::Result<()> {
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
pub async fn load_themes(editor: &mut Editor) -> anyhow::Result<()> {
    let themes_dir = themes_dir();
    let mut read = tokio::fs::read_dir(themes_dir).await?;

    while let Ok(Some(entry)) = read.next_entry().await {
        match themes::parse_theme(entry.path()).await {
            Ok(theme) => {
                tracing::info!("Loaded theme `{}` into memory", theme.id());
                editor.themes.push(theme);
            }
            Err(e) => tracing::error!("Could not load theme at `{}`: {e}", entry.path().display()),
        }
    }

    Ok(())
}
