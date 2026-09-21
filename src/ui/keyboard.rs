use crate::{editor::Editor, note::Mode, screen::Screen};

use dioxus::prelude::*;

#[store(pub)]
impl<Lens> Store<Editor, Lens> {
    async fn handle_key(&mut self, key: Key) -> bool {
        let mut editor = self.write();
        let result = editor
            .with_focused_screen_mut(|Screen::Note(note)| {
                let mode = note.mode;
                match key {
                    Key::Escape => {
                        if mode == Mode::Insert {
                            note.mode = Mode::Normal;
                        }
                        false
                    }
                    Key::Character(char) => {
                        println!("{}", char);
                        match mode {
                            Mode::Normal => {
                                if char == "i" {
                                    note.mode = Mode::Insert;
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
