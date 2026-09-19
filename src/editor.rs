use std::{collections::HashMap, sync::Arc};

use anyhow::bail;
use tokio::sync::{RwLock, RwLockMappedWriteGuard, RwLockReadGuard, RwLockWriteGuard};

use crate::{
    buffer::{Buffer, Note},
    config::Config,
    screen::{Screen, ScreenId},
};

#[derive(Default, Clone)]
pub struct Editor {
    pub config: Config,
    screens: Arc<RwLock<HashMap<ScreenId, Screen>>>,
    focus: Option<ScreenId>,
    next_id: u64,
}

#[derive(Clone)]
pub enum Mode {
    Normal,
    Insert,
    Select,
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

    pub async fn get_screen(&self, screen_id: ScreenId) -> Option<RwLockReadGuard<'_, Screen>> {
        let screens_r = self.screens.read().await;
        RwLockReadGuard::try_map(screens_r, |m| m.get(&screen_id)).ok()
    }

    pub async fn get_screen_mut(
        &self,
        screen_id: ScreenId,
    ) -> Option<RwLockMappedWriteGuard<'_, Screen>> {
        let screens_w = self.screens.write().await;
        RwLockWriteGuard::try_map(screens_w, |m| m.get_mut(&screen_id)).ok()
    }

    pub async fn get_focused_screen(&self) -> Option<RwLockReadGuard<'_, Screen>> {
        let focus = self.focus?;
        let screens_r = self.screens.read().await;
        RwLockReadGuard::try_map(screens_r, |m| m.get(&focus)).ok()
    }

    pub async fn get_focused_screen_mut(
        &mut self,
        screen_id: ScreenId,
    ) -> Option<RwLockMappedWriteGuard<'_, Screen>> {
        let focus = self.focus?;
        let screens_w = self.screens.write().await;
        RwLockWriteGuard::try_map(screens_w, |m| m.get_mut(&focus)).ok()
    }

    pub async fn with_screen<R>(
        &self,
        screen_id: ScreenId,
        f: impl FnOnce(&Screen) -> R,
    ) -> Option<R> {
        let screens_r = self.screens.read().await;
        screens_r.get(&screen_id).map(f)
    }

    pub async fn with_screen_mut<R>(
        &self,
        screen_id: ScreenId,
        f: impl FnOnce(&mut Screen) -> R,
    ) -> Option<R> {
        let mut screens_w = self.screens.write().await;
        screens_w.get_mut(&screen_id).map(f)
    }

    pub async fn with_focused_screen<R>(&self, f: impl FnOnce(&Screen) -> R) -> Option<R> {
        let focus = self.focus?;
        let screens_r = self.screens.read().await;
        screens_r.get(&focus).map(f)
    }

    pub async fn with_focused_screen_mut<R>(&self, f: impl FnOnce(&mut Screen) -> R) -> Option<R> {
        let focus = self.focus?;
        let mut screens_w = self.screens.write().await;
        screens_w.get_mut(&focus).map(f)
    }

    pub async fn add_screen(&mut self, screen: Screen) -> ScreenId {
        self.next_id += 1;
        let id = ScreenId(self.next_id);
        let mut screens_w = self.screens.write().await;
        screens_w.insert(id, screen);
        id
    }

    pub async fn focus_screen(&mut self, screen_id: ScreenId) -> anyhow::Result<()> {
        let screens_r = self.screens.read().await;
        if !screens_r.contains_key(&screen_id) {
            bail!("Screen id not found");
        }
        self.focus = Some(screen_id);

        Ok(())
    }

    pub fn set_mode(&mut self, mode: Mode) {}
}
