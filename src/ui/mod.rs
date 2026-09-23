pub mod engine;
pub mod keyboard;
pub mod note;
pub mod stores;

use crate::editor::Editor;
use dioxus::prelude::*;
use keyboard::HandleKey;

const GLOBAL_CSS: &str = include_str!("global.css");

pub fn open_editor(editor: Editor) {
    tracing::info!("Launching editor...");
    let builder = LaunchBuilder::new().with_context(editor);

    #[cfg(all(feature = "desktop", target_os = "linux"))]
    let builder = {
        use dioxus::desktop::{Config, WindowBuilder};
        let wayland = std::env::var_os("WAYLAND_DISPLAY").is_some();
        builder.with_cfg(
            Config::new()
                .with_window(
                    WindowBuilder::new()
                        .with_title("editor")
                        .with_decorations(!wayland),
                )
                .with_menu(None),
        )
    };

    builder.launch(App);
}

#[component]
fn App() -> Element {
    let initial = use_context::<Editor>();
    let mut editor = use_store(move || initial);
    use_context_provider(|| editor);

    rsx! {
        document::Style { "{GLOBAL_CSS}" }
        ThemeStyle {}

        div {
            class: "app",
            tabindex: 0,
            autofocus: true,

            onkeydown: move |evt| async move {
                let key = evt.data.key();
                if !editor.handle_key(evt.data.key()).await {
                    evt.prevent_default();
                }
            },

            note::Buffer {}
        }
    }
}

#[component]
fn ThemeStyle() -> Element {
    let editor = use_context::<Store<Editor>>();
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
