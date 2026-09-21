use dioxus::prelude::*;

use crate::{
    buffer::BufferStoreExt,
    syntax::{to_lines, tree::parse},
};

#[component]
pub fn Buffer(font_size: f32, buffer: Store<crate::buffer::Buffer>) -> Element {
    let mut input_ref = use_signal(|| None);

    let base = use_memo(move || {
        to_lines(
            &parse(&buffer.read().to_string()),
            &buffer.decorations().read(),
        )
    });

    let lines = base.read();
    rsx! {
        div {
            class: "engine-container",
            textarea {
                autofocus: true,
                onmounted: move |cx| {
                    input_ref.set(Some(cx.data()));
                },
                class: "engine-textarea",
                oninput: move |ev| async move {
                    println!("event: {ev:?}")
                }
            }
            div {
                class: "engine-model",
                onclick: move |ev| async move {
                    let p = ev.client_coordinates();
                    if let Some((line, col)) = hit_test(p.x, p.y).await {
                        println!("cursor -> line {line}, col {col}");
                    }
                    if let Some(element) = input_ref.read().clone() {
                        let _ = element.set_focus(true).await;
                    }
                },
                {lines.iter().enumerate().map(|(i, item)| rsx! {
                    div {
                        key: "{i}",
                        class: "engine-line",
                        "data-line": "{i}",
                        {item.iter().enumerate().map(|(i, piece)| rsx! {
                            span {
                                key: "{i}",
                                class: {piece.class()},
                                {piece.text.clone()}
                            }
                        })}
                    }
                })}
            }
        }

    }
}

const CARET_JS: &str = include_str!("caret.js");

async fn hit_test(x: f64, y: f64) -> Option<(usize, usize)> {
    let js = format!("{CARET_JS}\ndioxus.send(hitTest({x}, {y}));");
    let mut eval = document::eval(&js);
    eval.recv::<Option<(usize, usize)>>().await.ok().flatten()
}
