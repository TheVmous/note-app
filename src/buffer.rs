use std::{fmt::Display, io::Write};

use ropey::{Rope, iter::Lines};
use text_size::TextRange;

use crate::Result;

#[derive(Default, Clone, Debug)]
pub struct Buffer {
    text: Rope,
    cursor: TextRange,
}

impl Buffer {
    pub fn new(text: &str) -> Self {
        Buffer {
            text: Rope::from_str(text),
            cursor: TextRange::default(),
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

    pub fn content_equals(&self, t: impl Into<String>) -> bool {
        self.to_string() == t.into()
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

    pub fn apply(&mut self) {}

    pub fn write_to<T: Write>(&self, writer: &mut T) -> Result<()> {
        self.text.write_to(writer)?;
        Ok(())
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.text)
    }
}
