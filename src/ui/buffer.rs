use dioxus::prelude::*;

use super::EditorCtx;

#[component]
pub fn Buffer() -> Element {
    let mut editor = use_context::<EditorCtx>().core;

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
        } else {
            p { "Saved!" }
        }

        button {
            onclick: move |_| {
                let mut editor = editor.write();
                let buffer = editor.open_buffer.as_mut().unwrap();
                match buffer.save() {
                    Ok(_) => println!("all good saving"),
                    Err(e) => println!("{}", e),
                }
            },
            "save",
        }

        textarea {
            font_size: "{font_size}px",
            value: buffer.buffer,
            oninput: move |ev| {
                let mut editor = editor.write();
                let open_note = editor.open_note(note, focus)
                if let Some(buffer) = editor.get_focused_screen() {
                    buffer.buffer = ev.value();
                    buffer.saved = buffer.saved_content.as_ref().is_some_and(|c| c == &buffer.buffer);
                }
            }
        }
    }
}
