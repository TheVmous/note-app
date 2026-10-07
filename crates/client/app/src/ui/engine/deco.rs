use text_size::TextRange;

use crate::{
    buffer::Mode,
    cursor::Selection,
    syntax::{
        Piece,
        deco::{Decorations, Style},
    },
};

pub fn visible_decorations(mode: &Mode, sel: &Selection) -> Decorations {
    let mut all = Decorations::default();
    for cursor in sel.cursors() {
        let start = (cursor.head.min(cursor.anchor) as u32).into();
        let end = (cursor.head.max(cursor.anchor) as u32).into();
        if matches!(mode, Mode::Insert) {
            all.add(TextRange::new(start, end), Style::Cursor);
            continue;
        }
        all.add(TextRange::new(start, end), Style::Selection);
    }
    all
}

impl Piece {
    pub fn class(&self) -> String {
        let mut c = format!("tok-{:?}", self.kind);
        for s in &self.decos {
            c.push(' ');
            c.push_str(s.class());
        }
        c
    }
}

impl Style {
    pub fn class(self) -> &'static str {
        match self {
            Style::Selection => "deco-selection",
            Style::Cursor => "cursor-bar",
        }
    }
}
