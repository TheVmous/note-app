use text_size::TextRange;

use crate::{
    cursor::Selection,
    syntax::{
        Piece,
        deco::{Decorations, Style},
    },
};

pub fn visible_decorations(sel: &Selection) -> Decorations {
    let mut all = Decorations::default();
    for cursor in sel.cursors() {
        let start = (cursor.head.min(cursor.anchor) as u32).into();
        let end = (cursor.head.max(cursor.anchor) as u32).into();
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
        }
    }
}
