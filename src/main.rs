#![allow(non_snake_case)]

use crate::{
    buffer::{Buffer, Note},
    config::read_config,
    editor::Editor,
    ui::open_editor,
};

pub mod buffer;
pub mod cli;
pub mod cmd;
pub mod config;
pub mod editor;
pub mod screen;
pub mod ui;

fn main() {
    let options = cli::get_options();
    let config = read_config().expect("couldnt read config");
    let note = match options.file {
        Some(path) => Note::open(path.into()).expect("couldnt open buffer"),
        None => Note::blank(format!("untitled{}", config.syst.default_ext).into()),
    };
    let mut editor = Editor::new(config);
    editor.open_note(note, true);
    open_editor(editor);
}
