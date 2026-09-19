pub mod buffer;
pub mod keyboard;

use dioxus::prelude::*;

use crate::{Editor};

#[derive(Clone, Copy)]
pub struct EditorCtx {
    core: Signal<Editor>,
}

pub fn open_editor(editor: Editor) {
    LaunchBuilder::new().with_context(editor).launch(App);
}

#[component]
fn App() -> Element {
    let initial = use_context::<Editor>();
    use_context_provider(|| EditorCtx {
        core: Signal::new(initial),
    });

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
