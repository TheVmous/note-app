use crate::{
    buffer::{BufferOps, Mode},
    screen::Screen,
};

use super::EditorCtx;
use dioxus::prelude::*;

impl EditorCtx {
    pub async fn handle_key(&self, key: Key) -> bool {
        let result = self
            .core
            .peek()
            .clone()
            .with_focused_screen_mut(|Screen::Note(note)| {
                let mode = note.get_mode();
                println!("Mode: {mode:?}");
                match key {
                    Key::Escape => {
                        if mode == Mode::Insert {
                            note.set_mode(Mode::Normal);
                            println!("normal mode now!");
                        }
                        false
                    }
                    Key::Character(char) => {
                        println!("{}", char);
                        match mode {
                            Mode::Normal => {
                                if char == "i" {
                                    note.set_mode(Mode::Insert);
                                    println!("insert mode now!");
                                    return false;
                                }
                                println!("no insertion in Normal Mode!");
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
