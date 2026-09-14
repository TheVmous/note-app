mod buffer;

use dioxus::prelude::*;

use crate::Editor;

pub fn open_editor(editor: Editor) {
    LaunchBuilder::new().with_context(editor).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            tabindex: 0,
            autofocus: true,
            style: "width: 100vw; height: 100vh; outline: none;",

            onkeydown: move |evt| {
                println!("Pressed {}", evt.data.key())
            },

            {buffer::Buffer()}
        }
    }
}
