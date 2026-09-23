use dioxus::prelude::*;

use crate::{
    buffer::{Buffer, Mode},
    ui::keyboard::HandleKey,
};

impl<Lens> HandleKey for Store<Buffer, Lens>
where
    Lens: Writable<Target = Buffer> + Copy + 'static,
{
    async fn handle_key(&mut self, key: Key) -> bool {
        let buffer = self.peek();
        let mode = buffer.mode;
        drop(buffer);
        match key {
            Key::Escape => {
                if mode == Mode::Insert {
                    let mut buffer = self.write();
                    buffer.mode = Mode::Normal;
                }
                false
            }
            Key::Character(char) => {
                println!("{}", char);
                match mode {
                    Mode::Normal => {
                        if char == "i" {
                            let mut buffer = self.write();
                            buffer.mode = Mode::Insert;
                            return false;
                        }
                        false
                    }
                    _ => true,
                }
            }
            _ => true,
        }
    }
}
