use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::*;

mod deco;

use crate::{
    buffer::{BufferStoreExt, BufferStoreImplExt},
    cursor::Cursor,
    syntax::{deco::decorate, to_lines, tree::Grammar},
    ui::{
        engine::deco::visible_decorations,
        keyboard::{Direction, EditIntent, edit_intent},
    },
};

#[component]
pub fn Buffer(font_size: f32, buffer: Store<crate::buffer::Buffer>) -> Element {
    let mut input_ref = use_signal(|| None);

    let grammar = use_hook(|| Rc::new(RefCell::new(Grammar::default())));
    let base = use_memo(move || {
        let text = buffer.rope().to_string();
        to_lines(&grammar.borrow_mut().parse(&text))
    });
    let lines = use_memo(move || {
        let decos = visible_decorations(&buffer.selection().read());
        decorate(&base.read(), &decos)
    });

    rsx! {
        div { class: "engine-container",
            textarea {
                autofocus: true,
                class: "engine-textarea",
                onmounted: move |cx| {
                    input_ref.set(Some(cx.data()));
                    reset_input();
                },
                onbeforeinput: move |ev| {
                    println!("event: {}", ev.input_type());
                    ev.prevent_default();
                    match edit_intent(&ev) {
                        EditIntent::InsertText(data) => {
                            let mut buffer = buffer.write();
                            buffer.change(|c| {
                                (c.head, c.head, Some(data.clone()))
                            });
                            buffer.selection_mut().advance(data.len() as isize);
                        },
                        EditIntent::Delete { dir, .. } => {
                            let len = 1; // todo
                            let mut buffer = buffer.write();
                            buffer.change(|c| {
                                let from = c.head.saturating_add_signed(len * dir as isize);
                                (from, from + len as usize, None)
                            });
                            if matches!(dir, Direction::Backward) {
                                buffer.selection_mut().advance(-len);
                            }
                        }
                        o => {
                            println!("unsupported event {o:?}");
                        }
                    }
                    reset_input();
                },
            }
            div {
                class: "engine-model",
                onmousedown: move |_| async move {
                    if let Some(element) = input_ref() {
                        let _ = element.set_focus(true).await;
                        reset_input();
                    }
                },
                onclick: move |ev| async move {
                    let p = ev.client_coordinates();
                    if let Some(pos) = hit_test(p.x, p.y).await {
                        let cursor = Cursor {
                            anchor: pos,
                            head: pos + 1
                        };
                        buffer.selection().write().set(cursor);
                    }
                },
                for (i, line) in lines.read().iter().enumerate() {
                    div {
                        key: "{i}",
                        class: "engine-line",
                        "data-start": "{line.start}",
                        "data-end": "{line.end}",
                        for (j, piece) in line.pieces.iter().enumerate() {
                            span {
                                key: "{j}",
                                class: "{piece.class()}",
                                "data-start": "{piece.start}",
                                "{piece.text}"
                            }
                        }
                    }
                }
            }
        }
    }
}

const INPUT_JS: &str = include_str!("input.js");

fn reset_input() {
    spawn(async {
        let js = format!("{INPUT_JS}\ndioxus.send(resetInput());");
        let _ = document::eval(&js).recv::<bool>().await;
    });
}

const CARET_JS: &str = include_str!("caret.js");

async fn hit_test(x: f64, y: f64) -> Option<usize> {
    let js = format!("{CARET_JS}\ndioxus.send(hitTest({x}, {y}));");
    let mut eval = document::eval(&js);
    eval.recv::<Option<usize>>().await.ok().flatten()
}
