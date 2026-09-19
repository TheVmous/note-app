use dioxus::prelude::*;
use super::EditorCtx;

#[derive(Clone, PartialEq, Debug)]
pub enum KeyPress {
    Char(char),
    Enter,
    Esc,
    Backspace,
    
}

impl EditorCtx {
    fn handle_key(mut self, key: KeyPress) {
        let editor = self.core.write();
        match key {
            KeyPress::Esc => {
            }
            _ => ()
        }
    }
}