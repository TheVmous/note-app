use dioxus::prelude::*;

#[component]
pub fn Buffer(buffer: Store<crate::buffer::Buffer>) -> Element {
    let content = buffer.read();
    let lines = content.lines().map(|i| i.to_string());
    let mut input_ref = use_signal(|| None);

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
                    if let Some(element) = input_ref.read().clone() {
                        let _ = element.set_focus(true).await;
                    }
                },
                {lines.enumerate().map(|(i, item)| rsx! {
                    div {
                        key: "engine-line-{i}",
                        class: "engine-line",
                        onclick: move |ev| async move {
                            let _coords = ev.data().coordinates().element();
                            buffer.write().set_cursor_to(i, 1);
                        },
                        span { {item} }
                    }
                })}
            }
        }

    }
}
