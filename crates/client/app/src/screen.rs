use dioxus_stores::Store;
use enum_dispatch::enum_dispatch;

use crate::note::Note;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScreenId(pub u64);

#[enum_dispatch]
pub trait ScreenOps {
    fn title(&self) -> String;
}

#[derive(Clone, Store)]
#[enum_dispatch(ScreenOps)]
pub enum Screen {
    Note(Note),
}
