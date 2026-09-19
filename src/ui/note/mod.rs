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
        return rsx! {
            div { class: "note-empty", "No note open" }
        };
    };

    let name = note
        .path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "untitled".into());
    let dir = note
        .path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(|p| format!("{}/", p.display()));

    let lines = note.content.lines().count().max(1);
    let words = note.content.split_whitespace().count();
    let chars = note.content.chars().count();
    let mode = note.mode.to_string().to_uppercase();
    let state = if note.saved { "saved" } else { "dirty" };
    let state_label = if note.saved { "Saved" } else { "Unsaved" };

    rsx! {
        div { class: "note", style: "--note-font-size: {font_size}px",
            header { class: "note-bar",
                div { class: "note-title",
                    if let Some(dir) = dir {
                        span { class: "dir", "{dir}" }
                    }
                    span { class: "name", "{name}" }
                }
                div { class: "note-actions",
                    span { class: "note-state {state}", "{state_label}" }
                    button {
                        class: "note-save",
                        onclick: move |_| async move {
                            let core = editor.peek().clone();
                            core.with_focused_screen_mut(|Screen::Note(note)| match note.save() {
                                Ok(_) => println!("all good saving"),
                                Err(e) => println!("{}", e),
                            })
                            .await;
                            editor.write();
                        },
                        "Save"
                    }
                }
            }

            div { class: "note-body",
                textarea {
                    class: "note-editor",
                    spellcheck: false,
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

            footer { class: "note-status",
                div { class: "stats",
                    span { "{lines} lines" }
                    span { "{words} words" }
                    span { "{chars} chars" }
                }
                span { "{mode}" }
            }
        }
    }
}
