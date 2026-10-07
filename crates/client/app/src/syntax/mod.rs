use crate::syntax::{
    deco::Style,
    tree::{ResolvedNode, SyntaxKind},
};

pub mod deco;
pub mod tree;

#[derive(Clone, PartialEq)]
pub struct Piece {
    pub kind: SyntaxKind,
    pub text: String,
    pub start: u32,
    pub decos: Vec<Style>,
}

#[derive(Clone, PartialEq)]
pub struct Line {
    pub start: u32,
    pub end: u32,
    pub pieces: Vec<Piece>,
}

pub fn to_lines(root: &ResolvedNode) -> Vec<Line> {
    let mut lines = vec![Line {
        start: 0,
        end: 0,
        pieces: Vec::new(),
    }];
    for el in root.descendants_with_tokens() {
        let Some(tok) = el.as_token() else { continue };
        let start: u32 = tok.text_range().start().into();
        let end = start + tok.text().len() as u32;
        if tok.kind() == SyntaxKind::Newline {
            lines.last_mut().unwrap().end = start;
            lines.push(Line {
                start: end,
                end,
                pieces: Vec::new(),
            });
            continue;
        }
        let line = lines.last_mut().unwrap();
        line.end = end;
        line.pieces.push(Piece {
            kind: tok.kind(),
            text: tok.text().to_string(),
            start,
            decos: Vec::new(),
        });
    }
    lines
}
