use std::{
    fs::{File, OpenOptions},
    io::Read,
    path::PathBuf,
    sync::Arc,
};

use dioxus_stores::Store;
use text_size::TextRange;

use crate::{Result, buffer::Buffer, screen::ScreenOps};

#[derive(Default, Clone, Debug, Store)]
pub struct Note {
    pub path: PathBuf,
    pub content: Buffer,
    pub saved_content: Option<Arc<str>>,
    pub cursor: TextRange,
}

impl Note {
    pub fn blank(path: PathBuf) -> Self {
        Note {
            path,
            content: Buffer::default(),
            cursor: TextRange::default(),
            saved_content: None,
        }
    }

    pub fn open(path: PathBuf) -> Result<Note> {
        if !path.exists() {
            return Ok(Self::blank(path));
        }
        Self::read(path)
    }

    pub fn read(path: PathBuf) -> Result<Note> {
        let mut buffer = String::new();
        let mut file = File::open(&path)?;
        file.read_to_string(&mut buffer)?;

        Ok(Note {
            path,
            content: Buffer::new(&buffer),
            saved_content: Some(buffer.into()),
            cursor: TextRange::default(),
        })
    }

    pub fn save(&mut self) -> Result<()> {
        let mut file = OpenOptions::new().write(true).open(&self.path)?;
        self.content.write_to(&mut file)?;
        tracing::debug!("Saved file `{}`", self.path.display());
        self.saved_content = Some(self.content.to_string().into());
        Ok(())
    }
}

impl ScreenOps for Note {
    fn title(&self) -> String {
        "file".into()
    }
}
