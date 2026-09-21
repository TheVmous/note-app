use text_size::TextRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Red,
}

impl Style {
    pub fn class(self) -> &'static str {
        match self {
            Style::Red => "deco-red",
        }
    }
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

    pub fn highlight_red(&mut self, start: u32, end: u32) {
        self.add(TextRange::new(start.into(), end.into()), Style::Red);
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
