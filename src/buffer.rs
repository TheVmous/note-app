use std::path::PathBuf;

use text_size::TextRange;

#[derive(Default, Clone)]
pub struct Buffer {
    pub path: Option<PathBuf>,
    pub cursor: TextRange,
    pub saved: bool
}

impl Buffer {
    pub fn blank() -> Self {
        Buffer {
            path: None,
            cursor: TextRange::default(),
            saved: false
        }
    }
}