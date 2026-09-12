use crate::{buffer::Buffer, config::Config};

#[derive(Default, Clone)]
pub struct Editor {
    pub open_buffer: Option<Buffer>,
    pub config: Config,
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

    pub fn close_buffer(&mut self) -> bool {
        if self.open_buffer.is_none() {
            return false;
        }
        self.open_buffer = None;
        true
    }
}
