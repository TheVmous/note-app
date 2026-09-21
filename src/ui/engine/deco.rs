use crate::syntax::{Piece, deco::Style};

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
