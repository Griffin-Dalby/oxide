/*

    Oxide Client Ratatui App

    Griffin Dalby
    2025.10.30

    This will provide a struct & implementation for a ratatui app.

*/

// Implement
use std::{io, sync::Arc};
use std::collections::HashMap;

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

// Pages
mod page;
use page::{Page};

// Page
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum PageId {
    Splash,
}

impl Default for PageId {
    fn default() -> Self {
        PageId::Splash
    }
}

// Struct
#[derive(Debug, Default)]
pub struct AppState {
    pub username: Option<String>,
    pub messages: Vec<String>,
}

#[derive(Debug, Default)]
pub struct App {
    pub pages: page::PageMap,
    pub current_page: PageId,
    pub exit: bool,
    pub state: AppState,
}

impl App {
    pub fn new() -> Self {
        let pages = page::init();
    
        Self {
            pages,
            current_page: PageId::Splash,
            exit: false,
            state: AppState::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                let area = frame.area();
                let page = self.pages.get(&self.current_page).unwrap();
                let page = page.lock().unwrap();

                page.draw(frame);
            })?;

            if event::poll(std::time::Duration::from_millis(100))? {
                let evt = event::read()?;
                let page = self.pages.get_mut(&self.current_page).unwrap();
                let mut page = page.lock().unwrap();

                let _eres = page.handle_event(&evt, &mut self.state);
            }
        }
        Ok(())
    }
}