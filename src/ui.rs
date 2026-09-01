use dioxus::prelude::*;

use crate::Editor;

pub fn open_editor(editor: Editor) {
    use_context_provider(|| editor);
    dioxus::launch(App);
}

fn App() -> Element {
    let editor = use_context::<Editor>();
    rsx! {
        h1 { "The Editor" }
    }
}
