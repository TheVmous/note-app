#![allow(non_snake_case)]

use crate::ui::open_editor;

pub mod cli;
pub mod ui;

#[derive(Clone)]
pub struct Buffer {}

#[derive(Default, Clone)]
pub struct Editor {
    pub open_buffer: Option<Buffer>,
}

fn main() {
    let editor = Editor::default();
    open_editor(editor);
}
