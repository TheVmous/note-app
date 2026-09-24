use dioxus::prelude::*;

use crate::{
    buffer::{Buffer, Mode},
    ui::keyboard::{Direction, EditIntent, HandleKey, edit_intent},
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
                    buffer.set_mode(Mode::Normal)
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

pub fn handle_input(mut buffer: Store<Buffer>, ev: Event<BeforeInputData>) {
    match edit_intent(&ev) {
        EditIntent::InsertText(data) => {
            let mut buffer = buffer.write();
            buffer.change(|c| (c.head, c.head, Some(data.clone())));
            buffer.selection_mut().advance(data.len() as isize);
        }
        EditIntent::Delete { dir, .. } => {
            let len = 1; // todo
            let mut buffer = buffer.write();
            buffer.change(|c| {
                let from = c.head.saturating_add_signed(len * dir as isize);
                (from, from + len as usize, None)
            });
            if matches!(dir, Direction::Backward) {
                buffer.selection_mut().advance(-len);
            }
        }
        o => {
            println!("unsupported event {o:?}");
        }
    }
}
