use std::{fmt::Display, io::Write};

use dioxus::signals::{ReadSignal, Readable, UnsyncStorage};
use dioxus_stores::{ReadStore, Store, store};
use ropey::{Rope, iter::Lines};

use crate::{
    Result,
    cursor::{Cursor, Selection},
};

#[derive(Default, Clone, Debug, Store)]
pub struct Buffer {
    text: Rope,
    pub selection: Selection,
    pub mode: Mode,
}

impl Buffer {
    pub fn new(text: &str) -> Self {
        Buffer {
            text: Rope::from_str(text),
            ..Default::default()
        }
    }

    pub fn lines<'a>(&'a self) -> Lines<'a> {
        self.text.lines()
    }

    pub fn num_lines(&self) -> usize {
        self.text.len_lines()
    }

    pub fn num_chars(&self) -> usize {
        self.text.len_chars()
    }

    pub fn content_equals(&self, t: &str) -> bool {
        self.text == t
    }

    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    pub fn selection_mut(&mut self) -> &mut Selection {
        &mut self.selection
    }

    pub fn index_of(&self, line: usize, col: usize) -> usize {
        let offset = self.text.line_to_char(line);
        offset + col
    }

    pub fn num_words(&self) -> usize {
        let mut count = 0;
        let mut in_word = false;

        for chunk in self.text.chunks() {
            for c in chunk.chars() {
                if c.is_whitespace() {
                    in_word = false;
                } else if !in_word {
                    count += 1;
                    in_word = true;
                }
            }
        }

        count
    }

    pub fn apply(&mut self, tx: &Transaction) {
        tx.changes.apply(&mut self.text);
    }

    pub fn change(&mut self, map: impl Fn(&Cursor) -> Change) {
        self.apply(&self.mk_change(map));
    }

    pub fn mk_change(&self, map: impl Fn(&Cursor) -> Change) -> Transaction {
        Transaction::change(&self.text, self.selection.cursors().iter().map(map))
    }

    pub fn write_to<T: Write>(&self, writer: &mut T) -> Result<()> {
        self.text.write_to(writer)?;
        Ok(())
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Insert(String),
    Keep(usize),
    Delete(usize),
}

pub type Change = (usize, usize, Option<String>);

pub struct Changes {
    ops: Vec<Operation>,
}

impl Changes {
    pub fn apply(&self, content: &mut Rope) {
        let mut pos = 0;
        for operation in &self.ops {
            match operation {
                Operation::Insert(str) => {
                    content.insert(pos, str);
                    pos += str.len();
                }
                Operation::Keep(u) => {
                    pos += u;
                }
                Operation::Delete(num) => {
                    content.remove(pos..pos + num);
                }
            }
        }
    }
}

pub struct Transaction {
    changes: Changes,
    selection: Option<Selection>,
}

impl Transaction {
    pub fn change(doc: &Rope, changes: impl Iterator<Item = Change>) -> Self {
        let len = doc.len_chars();
        let mut ops = Vec::new();
        let mut pos = 0;

        for (from, to, replacement) in changes {
            if from > pos {
                ops.push(Operation::Keep(from - pos));
            }
            if to > from {
                ops.push(Operation::Delete(to - from));
            }
            if let Some(s) = replacement {
                ops.push(Operation::Insert(s));
            }
            pos = to;
        }
        if pos < len {
            ops.push(Operation::Keep(len - pos));
        }

        Transaction {
            changes: Changes { ops },
            selection: None,
        }
    }
}

#[store(pub)]
impl<Lens> Store<Buffer, Lens> {
    pub fn rope(&self) -> ReadStore<Rope>
    where
        Lens: Readable<Storage = UnsyncStorage>,
    {
        let map: fn(&Buffer) -> &Rope = |b| &b.text;
        let map_mut: fn(&mut Buffer) -> &mut Rope = |b| &mut b.text;
        self.into_selector()
            .child(0, map, map_mut)
            .map_writer(ReadSignal::from)
            .into()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, strum::Display)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Select,
}

mod tests {
    use crate::{buffer::Buffer, cursor::Cursor};

    #[test]
    pub fn test_transaction() {
        let mut buffer = Buffer::new("Hello World!");
        buffer.selection_mut().set(Cursor { anchor: 4, head: 5 });
        let tx = &buffer.mk_change(|c| (c.head, c.head, Some(",".to_string())));
        buffer.apply(tx);
        assert_eq!(buffer.to_string(), "Hello, World!")
    }
}
