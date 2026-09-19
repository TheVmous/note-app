use dioxus::prelude::*;

use super::EditorCtx;
use crate::{buffer::BufferOps, screen::Screen};

#[component]
pub fn Buffer() -> Element {
    let mut editor = use_context::<EditorCtx>().core;

    let font_size = editor.read().config.visuals.font_size;

    let note = use_resource(move || {
        let core = editor.read().clone();
        async move {
            core.with_focused_screen(|Screen::Note(note)| note.clone())
                .await
        }
    });

    let Some(Some(note)) = note() else {
        return rsx! { p { "no screen focused" } };
    };

    rsx! {
        h1 { "The Editor" }
        if !note.saved {
            p { "This is not saved" }
        } else {
            p { "Saved!" }
        }

        button {
            onclick: move |_| async move {
                let core = editor.peek().clone();
                core.with_focused_screen_mut(|Screen::Note(note)| match note.save() {
                    Ok(_) => println!("all good saving"),
                    Err(e) => println!("{}", e),
                })
                .await;
                editor.write();
            },
            "save",
        }

        textarea {
            font_size: "{font_size}px",
            value: "{note.content}",
            oninput: move |ev| async move {
                let core = editor.peek().clone();
                core.with_focused_screen_mut(|Screen::Note(note)| {
                    note.content = ev.value().into();
                    note.saved = note.saved_content.as_deref() == Some(&*note.content);
                })
                .await;
                editor.write();
            }
        }
    }
}
