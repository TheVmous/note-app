use dioxus::prelude::*;

#[component]
pub fn Buffer(buffer: Store<crate::buffer::Buffer>) -> Element {
    let buffer = buffer.read();
    let lines = buffer.lines()
        .map(|i| i.to_string());
    
    rsx! {
        div {
            class: "engine-container",
            textarea {
                class: "engine-textarea",
            }
            div {
                class: "engine-model",
                {lines.enumerate().map(|(i, item)| rsx! {
                    div {
                        key: "engine-line-{i}",
                        class: "engine-line",
                        span { {item} }
                    }
                })}
            }
        }

    }
}
