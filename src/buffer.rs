use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    sync::Arc,
    writeln,
};

use enum_dispatch::enum_dispatch;
use text_size::TextRange;
use thiserror::Error;

use crate::screen::ScreenOps;

#[enum_dispatch(BufferOps)]
pub enum Buffer {
    Note(Note),
}

#[enum_dispatch]
pub trait BufferOps {
    fn path(&self) -> &PathBuf;
    fn content(&self) -> Arc<str>;
    fn set_content(&mut self, content: Arc<str>);
    fn is_saved(&self) -> bool;
    fn save(&mut self) -> Result<(), BufferError>;
}

#[derive(Default, Clone, Debug)]
pub struct Note {
    pub path: PathBuf,
    pub content: Arc<str>,
    pub saved_content: Option<Arc<str>>,
    pub cursor: TextRange,
    pub saved: bool,
}

impl Note {
    pub fn blank(path: PathBuf) -> Self {
        Note {
            path,
            content: Arc::default(),
            cursor: TextRange::default(),
            saved_content: None,
            saved: false,
        }
    }

    pub fn open(path: PathBuf) -> Result<Note, BufferError> {
        if !path.exists() {
            return Ok(Self::blank(path));
        }
        Self::read(path)
    }

    pub fn read(path: PathBuf) -> Result<Note, BufferError> {
        let mut buffer = String::new();
        let mut file = File::open(&path)?;
        file.read_to_string(&mut buffer)?;
        let content: Arc<str> = buffer.into();

        Ok(Note {
            path,
            saved_content: Some(content.clone()),
            content,
            cursor: TextRange::default(),
            saved: true,
        })
    }
}

impl BufferOps for Note {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn content(&self) -> Arc<str> {
        self.content.clone()
    }

    fn is_saved(&self) -> bool {
        self.saved
    }

    fn save(&mut self) -> Result<(), BufferError> {
        let mut file = OpenOptions::new().write(true).open(&self.path)?;
        writeln!(&mut file, "{}", self.content)?; //fails here
        println!("{}", &self.path.display());
        self.saved_content = Some(self.content.clone());
        self.saved = true;
        Ok(())
    }

    fn set_content(&mut self, content: Arc<str>) {
        self.content = content;
    }
}

impl ScreenOps for Note {
    fn title(&self) -> String {
        "file".into()
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
