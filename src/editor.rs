use crate::buffer::Buffer;

#[derive(Default, Clone)]
pub struct Editor {
    pub open_buffer: Option<Buffer>,
}

impl Editor {
    pub fn open_buffer(buffer: Buffer) {
        
    }
}