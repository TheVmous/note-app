use text_size::TextRange;

use crate::syntax::{Line, Piece};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Selection,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Decoration {
    pub range: TextRange,
    pub style: Style,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Decorations {
    items: Vec<Decoration>,
}

impl Decorations {
    pub fn add(&mut self, range: TextRange, style: Style) {
        if !range.is_empty() {
            self.items.push(Decoration { range, style });
        }
    }

    pub fn items(&self) -> &[Decoration] {
        &self.items
    }

    pub fn clear(&mut self, style: Style) {
        self.items.retain(|d| d.style != style);
    }

    pub fn apply_edit(&mut self, at: u32, removed: u32, inserted: u32) {
        let end = at + removed;
        let map = |p: u32| {
            if p <= at {
                p
            } else if p >= end {
                p - removed + inserted
            } else {
                at
            }
        };
        for d in &mut self.items {
            let (s, e) = (map(d.range.start().into()), map(d.range.end().into()));
            d.range = TextRange::new(s.into(), e.into());
        }
        self.items.retain(|d| !d.range.is_empty());
    }
}

pub fn decorate(lines: &[Line], decos: &Decorations) -> Vec<Line> {
    lines
        .iter()
        .map(|line| Line {
            start: line.start,
            end: line.end,
            pieces: line
                .pieces
                .iter()
                .flat_map(|p| split_piece(p, decos))
                .collect(),
        })
        .collect()
}

fn split_piece(p: &Piece, decos: &Decorations) -> Vec<Piece> {
    let end = p.start + p.text.len() as u32;
    let mut cuts = vec![p.start, end];
    for d in &decos.items {
        for b in [u32::from(d.range.start()), u32::from(d.range.end())] {
            if b > p.start && b < end && p.text.is_char_boundary((b - p.start) as usize) {
                cuts.push(b);
            }
        }
    }
    cuts.sort_unstable();
    cuts.dedup();

    cuts.windows(2)
        .map(|w| {
            let (a, b) = (w[0], w[1]);
            Piece {
                kind: p.kind,
                text: p.text[(a - p.start) as usize..(b - p.start) as usize].to_string(),
                start: a,
                decos: decos
                    .items
                    .iter()
                    .filter(|d| u32::from(d.range.start()) <= a && b <= u32::from(d.range.end()))
                    .map(|d| d.style)
                    .collect(),
            }
        })
        .collect()
}
