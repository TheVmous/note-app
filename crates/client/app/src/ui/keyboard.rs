use crate::editor::Editor;

use dioxus::prelude::*;

pub trait HandleKey {
    async fn handle_key(&mut self, key: Key) -> bool;
}

impl<Lens> HandleKey for Store<Editor, Lens>
where
    Lens: Writable<Target = Editor> + Copy + 'static,
{
    async fn handle_key(&mut self, _key: Key) -> bool {
        true
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

        DeleteContentBackward => EditIntent::Delete {
            dir: Backward,
            unit: Char,
        },
        DeleteContentForward => EditIntent::Delete {
            dir: Forward,
            unit: Char,
        },
        DeleteWordBackward => EditIntent::Delete {
            dir: Backward,
            unit: Word,
        },
        DeleteWordForward => EditIntent::Delete {
            dir: Forward,
            unit: Word,
        },
        DeleteSoftLineBackward | DeleteHardLineBackward => EditIntent::Delete {
            dir: Backward,
            unit: Line,
        },
        DeleteSoftLineForward | DeleteHardLineForward => EditIntent::Delete {
            dir: Forward,
            unit: Line,
        },
        DeleteEntireSoftLine => EditIntent::DeleteLine,
        DeleteContent | DeleteByCut | DeleteByDrag => EditIntent::DeleteSelection,

        HistoryUndo => EditIntent::Undo,
        HistoryRedo => EditIntent::Redo,

        other => EditIntent::Unsupported(other),
    }
}
