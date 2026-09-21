use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::*;

mod deco;

use crate::{
    buffer::BufferStoreExt,
    cursor::Cursor,
    syntax::{deco::decorate, to_lines, tree::Grammar},
    ui::engine::deco::visible_decorations,
};

#[component]
pub fn Buffer(font_size: f32, buffer: Store<crate::buffer::Buffer>) -> Element {
    let mut input_ref = use_signal(|| None);

    let grammar = use_hook(|| Rc::new(RefCell::new(Grammar::default())));
    let base = use_memo(move || {
        let text = buffer.text().read().to_string();
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
                onmounted: move |cx| input_ref.set(Some(cx.data())),
                oninput: move |ev| println!("event: {ev:?}"),
            }
            div {
                class: "engine-model",
                onclick: move |ev| async move {
                    let p = ev.client_coordinates();
                    if let Some(pos) = hit_test(p.x, p.y).await {
                        let cursor = Cursor {
                            anchor: pos,
                            head: pos+1
                        };
                        buffer.selection().write().set(cursor);
                    }
                    if let Some(element) = input_ref() {
                        let _ = element.set_focus(true).await;
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

const CARET_JS: &str = include_str!("caret.js");

async fn hit_test(x: f64, y: f64) -> Option<usize> {
    let js = format!("{CARET_JS}\ndioxus.send(hitTest({x}, {y}));");
    let mut eval = document::eval(&js);
    eval.recv::<Option<usize>>().await.ok().flatten()
}
