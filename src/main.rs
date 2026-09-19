#![allow(non_snake_case)]

use tokio::fs::ReadDir;

use crate::{
    buffer::{Buffer, Note},
    config::read_config,
    editor::Editor,
    fs::themes_dir,
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
    let config = read_config().expect("couldnt read config");
    let note = match options.file {
        Some(path) => Note::open(path.into()).expect("couldnt open buffer"),
        None => Note::blank(format!("untitled{}", config.syst.default_ext).into()),
    };
    let mut editor = Editor::new(config);
    let runtime = tokio::runtime::Runtime::new().expect("tokio runtime");
    runtime.block_on(reload_editor(&mut editor));
    runtime.block_on(editor.open_note(note, true));
    open_editor(editor);
}

pub async fn reload_editor(editor: &mut Editor) {
    load_themes(editor).await;
}

pub async fn load_themes(editor: &mut Editor) -> anyhow::Result<()> {
    let themes_dir = themes_dir();
    println!("reading themes {}", themes_dir.display());
    let mut read = tokio::fs::read_dir(themes_dir).await?;

    while let Ok(Some(entry)) = read.next_entry().await {
        let Ok(theme) = themes::parse_theme(entry.path()).await else {
            // todo: log error
            continue;
        };
        println!("loaded theme!");
        editor.themes.push(theme);
    }

    Ok(())
}
