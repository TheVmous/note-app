use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    writeln,
};

use text_size::TextRange;
use thiserror::Error;

#[derive(Default, Clone, Debug)]
pub struct Buffer {
    pub path: PathBuf,
    pub content: String,
    pub saved_content: Option<String>,
    pub cursor: TextRange,
    pub saved: bool,
}

impl Buffer {
    pub fn blank(path: PathBuf) -> Self {
        Buffer {
            path,
            content: String::new(),
            cursor: TextRange::default(),
            saved_content: None,
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
            saved_content: Some(buffer.clone()),
            content: buffer,
            cursor: TextRange::default(),
            saved: true,
        })
    }
    //check for perms here via sm std or other function
    pub fn save(&mut self) -> Result<(), BufferError> {
        let mut file = File::open(&self.path)?;
        writeln!(&mut file, "{}", self.content)?;
        self.saved_content = Some(self.content.clone());
        self.saved = true;
        Ok(())
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
