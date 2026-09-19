use std::{collections::HashMap, sync::Arc};

use anyhow::bail;
use tokio::sync::RwLock;

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

    pub async fn get_screen(&self, screen_id: ScreenId) -> Option<&Screen> {
        let screens_r = self.screens.read().await;
        screens_r.get(&screen_id)
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
