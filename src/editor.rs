use std::collections::HashMap;

use anyhow::bail;
use dioxus::{
    prelude::{ReadableExt, UnsyncStorage, Writable},
    stores::{Store, store},
};

use crate::{
    Result,
    config::Config,
    note::Note,
    screen::{Screen, ScreenId, ScreenStoreExt},
    themes::Theme,
};

#[derive(Default, Clone, Store)]
pub struct Editor {
    pub config: Config,
    pub themes: Vec<Theme>,
    theme_id: Option<String>,
    screens: HashMap<ScreenId, Screen>,
    focus: Option<ScreenId>,
    next_id: u64,
}

impl Editor {
    pub fn new(config: Config) -> Editor {
        Editor {
            config,
            ..Default::default()
        }
    }

    pub async fn open_note(&mut self, note: Note, focus: bool) {
        let screen = Screen::Note(note);
        let id = self.add_screen(screen).await;
        if focus {
            self.focus_screen(id).await.expect("immediately closed?")
        }
    }

    pub async fn get_screen(&self, screen_id: ScreenId) -> Option<&Screen> {
        self.screens.get(&screen_id)
    }

    pub async fn get_screen_mut(&mut self, screen_id: ScreenId) -> Option<&mut Screen> {
        self.screens.get_mut(&screen_id)
    }

    pub async fn focused_screen(&self) -> Option<&Screen> {
        self.screens.get(self.focus.as_ref()?)
    }

    pub async fn focused_screen_mut(&mut self) -> Option<&mut Screen> {
        self.screens.get_mut(self.focus.as_ref()?)
    }

    pub async fn with_screen<R>(
        &self,
        screen_id: ScreenId,
        f: impl FnOnce(&Screen) -> R,
    ) -> Option<R> {
        self.get_screen(screen_id).await.map(f)
    }

    pub async fn with_screen_mut<R>(
        &mut self,
        screen_id: ScreenId,
        f: impl FnOnce(&mut Screen) -> R,
    ) -> Option<R> {
        self.get_screen_mut(screen_id).await.map(f)
    }

    pub async fn with_focused_screen<R>(&self, f: impl FnOnce(&Screen) -> R) -> Option<R> {
        self.focused_screen().await.map(f)
    }

    pub async fn with_focused_screen_mut<R>(
        &mut self,
        f: impl FnOnce(&mut Screen) -> R,
    ) -> Option<R> {
        self.focused_screen_mut().await.map(f)
    }

    pub async fn add_screen(&mut self, screen: Screen) -> ScreenId {
        self.next_id += 1;
        let id = ScreenId(self.next_id);
        self.screens.insert(id, screen);
        id
    }

    pub async fn focus_screen(&mut self, screen_id: ScreenId) -> Result<()> {
        if !self.screens.contains_key(&screen_id) {
            bail!("Screen id not found");
        }
        self.focus = Some(screen_id);

        Ok(())
    }

    pub async fn focused_note(&self) -> Option<&Note> {
        let w = self.focused_screen().await?;
        match w {
            Screen::Note(n) => Some(n),
        }
    }

    pub fn active_theme(&self) -> Option<&Theme> {
        let id = self.theme_id.as_deref()?;
        self.themes.iter().find(|t| t.id() == id)
    }

    pub fn set_theme(&mut self, theme_id: impl Into<String>) -> Result<()> {
        let theme_id = theme_id.into();
        if !self.themes.iter().any(|t| t.id() == theme_id) {
            bail!("Theme `{theme_id}` not loaded");
        }
        self.theme_id = Some(theme_id);
        Ok(())
    }

    pub fn clear_theme(&mut self) {
        self.theme_id = None;
    }
}

#[store(pub name = EditorFocusExt)]
impl<Lens> Store<Editor, Lens> {
    pub fn focused_note(&mut self) -> Option<Store<Note>>
    where
        Lens: Writable<Storage = UnsyncStorage>,
    {
        let focus = (*self.focus().read())?;
        let note = self.screens().get(focus)?.note()?;
        Some(note.into())
    }
}
