use std::{fs::File, io::Read, path::PathBuf};

use text_size::TextRange;
use thiserror::Error;

#[derive(Default, Clone)]
pub struct Buffer {
    pub path: Option<PathBuf>,
    pub content: String,
    pub cursor: TextRange,
    pub saved: bool,
}

impl Buffer {
    pub fn blank() -> Self {
        Buffer {
            path: None,
            content: String::new(),
            cursor: TextRange::default(),
            saved: false,
        }
    }
}

#[derive(Error, Debug)]
pub enum BufferError {
    #[error("io operation error: {io}")]
    IoError {
        #[from]
        io: std::io::Error,
    },
}

pub fn read_buffer(path: PathBuf) -> Result<Buffer, BufferError> {
    let mut buffer = String::new();
    let mut file = File::open(&path)?;
    file.read_to_string(&mut buffer);

    Ok(Buffer {
        path: Some(path),
        content: buffer,
        cursor: TextRange::default(),
        saved: true,
    })
}
