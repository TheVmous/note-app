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

    pub fn set(&mut self, cursor: Cursor) {
        self.cursors.clear();
        self.add(cursor);
    }

    pub fn collapse(&mut self) {
        self.cursors.drain(0..self.cursors.len() - 1);
        self.primary_index = 0;
    }

    pub fn cursors(&self) -> &[Cursor] {
        &self.cursors
    }

    pub fn lines(&self) -> Vec<usize> {
        self.cursors()
            .iter()
            .flat_map(|c| [c.anchor, c.head])
            .collect::<Vec<usize>>()
    }
}
