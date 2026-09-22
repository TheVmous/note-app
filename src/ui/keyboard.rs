use crate::{editor::Editor, note::Mode, screen::Screen};

use dioxus::prelude::*;

#[store(pub)]
impl<Lens> Store<Editor, Lens> {
    pub async fn handle_key(&mut self, key: Key) -> bool {
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

use dioxus::events::{BeforeInputData, InputType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Backward = -1,
    Forward = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Char,
    Word,
    Line,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditIntent {
    InsertText(String),
    Composition(String),
    Newline,
    Paste,
    DeleteSelection,
    Delete { dir: Direction, unit: Unit },
    DeleteLine,
    Undo,
    Redo,
    Unsupported(InputType),
}

pub fn edit_intent(input: &BeforeInputData) -> EditIntent {
    use Direction::*;
    use InputType::*;
    use Unit::*;

    let text = || input.data().unwrap_or_default();

    match input.input_type() {
        InsertText | InsertReplacementText => EditIntent::InsertText(text()),
        InsertCompositionText => EditIntent::Composition(text()),
        InsertParagraph | InsertLineBreak => EditIntent::Newline,
        InsertFromPaste | InsertFromPasteAsQuotation | InsertFromDrop | InsertFromYank => {
            EditIntent::Paste
        }

        DeleteContentBackward => EditIntent::Delete { dir: Backward, unit: Char },
        DeleteContentForward => EditIntent::Delete { dir: Forward, unit: Char },
        DeleteWordBackward => EditIntent::Delete { dir: Backward, unit: Word },
        DeleteWordForward => EditIntent::Delete { dir: Forward, unit: Word },
        DeleteSoftLineBackward | DeleteHardLineBackward => {
            EditIntent::Delete { dir: Backward, unit: Line }
        }
        DeleteSoftLineForward | DeleteHardLineForward => {
            EditIntent::Delete { dir: Forward, unit: Line }
        }
        DeleteEntireSoftLine => EditIntent::DeleteLine,
        DeleteContent | DeleteByCut | DeleteByDrag => EditIntent::DeleteSelection,

        HistoryUndo => EditIntent::Undo,
        HistoryRedo => EditIntent::Redo,

        other => EditIntent::Unsupported(other),
    }
}
