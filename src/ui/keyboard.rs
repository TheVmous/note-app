use super::EditorCtx;
use dioxus::prelude::*;

// #[derive(Clone, PartialEq, Debug)]
// pub enum KeyPress {
//     Char(char),
//     Enter,
//     Esc,
//     Backspace,
// }

impl EditorCtx {
    pub fn handle_key(mut self, key: Key) {
        // let editor = self.core.write();
        match key {
            Key::Character(char) => println!("{}", char),
            // Key::Escape() => 
            others => (),
        }
    }
}
