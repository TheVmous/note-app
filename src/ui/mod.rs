mod buffer;

use dioxus::prelude::*;

use crate::Editor;

pub fn open_editor(editor: Editor) {
    LaunchBuilder::new().with_context(editor).launch(App);
}

#[component]
fn App() -> Element {
    buffer::Buffer()
}
