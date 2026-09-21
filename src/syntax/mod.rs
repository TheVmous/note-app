use crate::syntax::{
    deco::{Decorations, Style},
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

pub fn to_lines(root: &ResolvedNode, decos: &Decorations) -> Vec<Vec<Piece>> {
    let mut lines = vec![Vec::new()];
    for el in root.descendants_with_tokens() {
        let Some(tok) = el.as_token() else { continue };
        if tok.kind() == SyntaxKind::Newline {
            lines.push(Vec::new());
            continue;
        }
        let text = tok.text();
        let start: u32 = tok.text_range().start().into();
        let end = start + text.len() as u32;

        let mut cuts = vec![start, end];
        for d in decos.items() {
            for p in [u32::from(d.range.start()), u32::from(d.range.end())] {
                if p > start && p < end && text.is_char_boundary((p - start) as usize) {
                    cuts.push(p);
                }
            }
        }
        cuts.sort_unstable();
        cuts.dedup();

        for w in cuts.windows(2) {
            let (a, b) = (w[0], w[1]);
            let styles = decos
                .items()
                .iter()
                .filter(|d| u32::from(d.range.start()) <= a && b <= u32::from(d.range.end()))
                .map(|d| d.style)
                .collect();
            lines.last_mut().unwrap().push(Piece {
                kind: tok.kind(),
                text: text[(a - start) as usize..(b - start) as usize].to_string(),
                start: a,
                decos: styles,
            });
        }
    }
    lines
}
