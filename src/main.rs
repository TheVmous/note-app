#![allow(non_snake_case)]

use crate::{editor::Editor, ui::open_editor};

pub mod buffer;
pub mod cli;
pub mod editor;
pub mod ui;

fn main() {
    let cli_options = cli::get_options();
    let editor = Editor::default();
    open_editor(editor);
}
