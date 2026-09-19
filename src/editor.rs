use crate::{buffer::Buffer, config::Config};

#[derive(Default, Clone)]
pub struct Editor {
    pub open_buffer: Option<Buffer>,
    pub config: Config,
}

#[derive(Clone)]
pub enum Mode {
    Normal,
    Insert,
    Select,
}

impl Editor {
    pub fn new(config: Config) -> Editor {
        Editor {
            config,
            ..Default::default()
        }
    }

    pub fn open_buffer(&mut self, buffer: Buffer) {
        self.open_buffer = Some(buffer);
    }

    pub fn set_mode(&mut self, mode: Mode) {
        
    }

    pub fn close_buffer(&mut self) -> bool {
        if self.open_buffer.is_none() {
            return false;
        }
        self.open_buffer = None;
        true
    }
}
