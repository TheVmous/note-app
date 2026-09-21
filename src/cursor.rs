#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cursor {
    pub anchor: usize,
    pub head: usize,
}

impl Cursor {}

#[derive(Default, Clone, Debug)]
pub struct Selection {
    cursors: Vec<Cursor>,
    primary_index: usize,
}

impl Selection {
    pub fn primary(&self) -> Cursor {
        self.cursors[self.primary_index]
    }

    pub fn add(&mut self, cursor: Cursor) {
        self.cursors.push(cursor);
        self.primary_index = self.cursors.len() - 1;
    }

    pub fn cursors(&self) -> &[Cursor] {
        &self.cursors
    }
}
