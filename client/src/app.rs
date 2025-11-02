/*

    Oxide Client Ratatui App

    Griffin Dalby
    2025.10.30

    This will provide a struct & implementation for a ratatui app.

*/

// Implement
use std::io;
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
use crate::Page::splash::SplashPage;


// Page
pub trait Page: std::fmt::Debug {
    fn draw(&self, frame: &mut Frame);
    fn handle_events(&mut self, event: &Event) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug)]
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
    pub pages: HashMap<PageId, Box<dyn Page>>,
    pub current_page: PageId,
    pub exit: bool,
    pub state: AppState,
}

impl App {
    pub fn new() -> Self {
        let mut pages: HashMap<PageId, Box<dyn Page>> = HashMap::new();
        pages.insert(PageId::Splash, Box::new(crate::::SplashPage::new()));
    
        Self {
            pages,
            current_page: PageId::Splash,
            exit: false,
            state: AppState::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        todo!()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
}