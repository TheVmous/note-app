use dioxus::prelude::*;

use crate::Editor;

pub fn open_editor(editor: Editor) {
    LaunchBuilder::new().with_context(editor).launch(App);
}

#[component]
fn App() -> Element {
    let editor = use_context::<Editor>();
    let buffer = editor.open_buffer.expect("for now buffers are mandatory");
    
    rsx! {
        h1 { "The Editor" }
        textarea {
            { buffer.content }
        }
    }
}
