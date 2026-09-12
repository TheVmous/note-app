#![allow(non_snake_case)]

use crate::{buffer::Buffer, config::read_config, editor::Editor, ui::open_editor};

pub mod buffer;
pub mod cli;
pub mod config;
pub mod editor;
pub mod ui;

const CHEATSHEET: &str = "Quick Keybinds:

q - Quit.
w - Save changes.
a - Add task.
x - Save and Quit.
u - Undo action.
U - Redo action.
H - Toggle cheatsheet.
Arrow Keys/Helix Motions - Move around.";

fn main() {
    let options = cli::get_options();
    let mut editor = Editor::default();
    let config = read_config();
    if let Some(buffer) = options.file {
        let buffer = Buffer::open(buffer.into()).expect("couldnt open buffer");
        editor.open_buffer(buffer);
    }
    open_editor(editor);
}
