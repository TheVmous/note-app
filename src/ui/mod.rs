pub mod keyboard;
pub mod note;

use crate::Buffer;
use crate::Editor;
use crate::screen::Screen;
use dioxus::prelude::*;

const GLOBAL_CSS: &str = include_str!("global.css");
#[derive(Clone, Copy)]
pub struct EditorCtx {
    core: Signal<Editor>,
}

pub fn open_editor(editor: Editor) {
    LaunchBuilder::new().with_context(editor).launch(App);
}

#[component]
fn App() -> Element {
    let initial = use_context::<Editor>();

    use_context_provider(|| EditorCtx {
        core: Signal::new(initial),
    });
    let mut editor_ctx = use_context::<EditorCtx>(); //crashes

    rsx! {
        document::Style { "{GLOBAL_CSS}" }
        ThemeStyle {}

        div {
            class: "app",
            tabindex: 0,
            autofocus: true,

            onkeydown: move |evt| async move {
                if !editor_ctx.handle_key(evt.data.key()).await {
                    evt.prevent_default();
                }
            },

            note::Buffer {}
        }
    }
}

#[component]
fn ThemeStyle() -> Element {
    let editor = use_context::<EditorCtx>().core;
    let css = use_memo(move || {
        editor
            .read()
            .active_theme()
            .map(|theme| theme.css().to_owned())
    });

    rsx! {
        if let Some(css) = css() {
            style { "{css}" }
        }
    }
}
