use dioxus::prelude::*;

use crate::Editor;

pub fn open_editor(editor: Editor) {
    LaunchBuilder::new().with_context(editor).launch(App);
}

#[component]
fn App() -> Element {
    let initial = use_context::<Editor>();
    let mut editor = use_signal(|| initial);

    let editor_read_guard = editor.read();
    let buffer = editor_read_guard
        .open_buffer
        .clone()
        .expect("for now buffers are mandatory");

    let font_size = editor_read_guard.config.visuals.font_size;
    rsx! {
        h1 { "The Editor" }
        if !buffer.saved {
            p { "This is not saved" }
        }
        textarea {
            font_size: "{font_size}px",
            value: buffer.content,
            oninput: move |ev| {
                let mut editor = editor.write();
                if let Some(buffer) = editor.open_buffer.as_mut() {
                    buffer.content = ev.value();
                    buffer.saved = buffer.saved_content.as_ref().is_some_and(|c| c == &buffer.content);
                }
            }
        }
    }
}
