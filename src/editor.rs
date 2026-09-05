use crate::buffer::Buffer;

#[derive(Default, Clone)]
pub struct Editor {
    pub open_buffer: Option<Buffer>,
}

impl Editor {
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
