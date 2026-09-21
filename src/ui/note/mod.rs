use dioxus::prelude::*;

use crate::{
    editor::{Editor, EditorFocusExt, EditorStoreExt},
    note::{Note, NoteStoreExt},
    ui::engine,
};

#[component]
pub fn Buffer() -> Element {
    let mut editor = use_context::<Store<Editor>>();
    let font_size = editor.config().read().visuals.font_size;

    let Some(note): Option<Store<Note>> = editor.focused_note() else {
        return rsx! {
            div { class: "note-empty", "No note open" }
        };
    };

    let (name, dir) = {
        let path = note.path();
        let path = path.read();
        (
            path.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "untitled".into()),
            path.parent()
                .filter(|p| !p.as_os_str().is_empty())
                .map(|p| format!("{}/", p.display())),
        )
    };

    let (lines, words, chars) = {
        let content = note.content();
        let content = content.read();
        (content.num_lines(), content.num_words(), content.num_chars())
    };
    let mode = note.mode().read().to_string().to_uppercase();
    let saved = false; // todo: fix
    let state = if saved { "saved" } else { "dirty" };
    let state_label = if saved { "Saved" } else { "Unsaved" };

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
                        onclick: move |_| {
                            let mut note = note;
                            match note.write().save() {
                                Ok(_) => println!("all good saving"),
                                Err(e) => println!("{}", e),
                            }
                        },
                        "Save"
                    }
                }
            }

            div { class: "note-body",
                engine::Buffer { buffer: note.content() }
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
