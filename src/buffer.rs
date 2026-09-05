use std::{fs::File, io::Read, path::PathBuf};

use text_size::TextRange;
use thiserror::Error;

#[derive(Default, Clone)]
pub struct Buffer {
    pub path: PathBuf,
    pub content: String,
    pub cursor: TextRange,
    pub saved: bool,
}

impl Buffer {
    pub fn blank(path: PathBuf) -> Self {
        Buffer {
            path,
            content: String::new(),
            cursor: TextRange::default(),
            saved: false,
        }
    }

    pub fn open(path: PathBuf) -> Result<Buffer, BufferError> {
        if !path.exists() {
            return Ok(Self::blank(path));
        }
        Self::read(path)
    }

    pub fn read(path: PathBuf) -> Result<Buffer, BufferError> {
        let mut buffer = String::new();
        let mut file = File::open(&path)?;
        file.read_to_string(&mut buffer)?;

        Ok(Buffer {
            path,
            content: buffer,
            cursor: TextRange::default(),
            saved: true,
        })
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
