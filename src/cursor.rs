#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cursor {
    pub anchor: usize,
    pub head: usize,
}

impl Cursor {
    pub fn shrink_head(&mut self) {
        self.head = self.anchor
    }

    pub fn shrink_anchor(&mut self) {
        self.anchor = self.head
    }
}

#[derive(Clone, Debug)]
pub struct Selection {
    cursors: Vec<Cursor>,
    primary_index: usize,
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            cursors: vec![Cursor { head: 0, anchor: 0 }],
            primary_index: 0,
        }
    }
}

impl Selection {
    pub fn primary(&self) -> Cursor {
        self.cursors[self.primary_index]
    }

    pub fn add(&mut self, cursor: Cursor) {
        self.cursors.push(cursor);
        self.primary_index = self.cursors.len() - 1;
    }

    pub fn advance(&mut self, delta: isize) {
        println!("old: {self:?}");
        for cursor in &mut self.cursors {
            cursor.anchor = cursor.anchor.saturating_add_signed(delta);
            cursor.head = cursor.head.saturating_add_signed(delta);
        }
        println!("new: {self:?}")
    }

    pub fn set(&mut self, cursor: Cursor) {
        self.cursors.clear();
        self.add(cursor);
    }

    pub fn collapse(&mut self) {
        self.cursors.drain(0..self.cursors.len() - 1);
        self.primary_index = 0;
    }

    pub fn shrink_heads(&mut self) {
        for cursor in &mut self.cursors {
            cursor.shrink_head();
        }
    }

    pub fn shrink_anchors(&mut self) {
        for cursor in &mut self.cursors {
            cursor.shrink_anchor();
        }
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
