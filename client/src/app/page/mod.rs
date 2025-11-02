use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fmt::Debug;

pub mod splash;
use crate::app::{AppState, PageId};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

pub trait Page: Debug + Send + Sync {
    fn draw(&self, frame: &mut Frame);
    fn handle_event(&mut self, event: &Event, app_state: &mut AppState) -> Result<(), Box<dyn std::error::Error>>;

    fn id(&self) -> &'static str;
    fn title(&self) -> &'static str;
}

// Functions

/* init()
  This function will register page behavior & return a sorted page list. */
pub type PageMap = HashMap<PageId, Arc<Mutex<dyn Page>>>;
pub fn init() -> PageMap {
    let mut pages: PageMap = HashMap::new();
    let splash = Arc::new(Mutex::new(splash::SplashPage::default()));

    pages.insert(PageId::Splash, splash);    
    pages
}
