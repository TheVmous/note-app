use enum_dispatch::enum_dispatch;

use crate::buffer::Note;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScreenId(pub u64);

#[enum_dispatch]
pub trait ScreenOps {
    fn title(&self) -> String;
}

#[enum_dispatch(ScreenOps)]
#[derive(Clone)]
pub enum Screen {
    Note(Note),
}
