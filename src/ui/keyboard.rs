use crate::{
    buffer::{BufferOps, Mode},
    screen::Screen,
};

use super::EditorCtx;
use dioxus::prelude::*;

impl EditorCtx {
    pub async fn handle_key(&mut self, key: Key) -> bool {
        let editor = self.core.write();
        let result = editor
            .with_focused_screen_mut(|Screen::Note(note)| {
                let mode = note.get_mode();
                match key {
                    Key::Escape => {
                        if mode == Mode::Insert {
                            note.set_mode(Mode::Normal);
                        }
                        false
                    }
                    Key::Character(char) => {
                        println!("{}", char);
                        match mode {
                            Mode::Normal => {
                                if char == "i" {
                                    note.set_mode(Mode::Insert);
                                    return false;
                                }
                                false
                            }
                            _ => true,
                        }
                    }
                    _ => true,
                }
            })
            .await;
        match result {
            None => {
                println!("Not an applicable screen!");
                false
            }
            Some(result) => result,
        }
    }
}
