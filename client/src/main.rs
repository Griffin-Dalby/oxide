/*

    Oxide Client Main

    Griffin Dalby
    2025.10.30

    This script will control the oxide client.

*/

// Implement
use std::io;

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

use std::env;
use std::error::Error;

mod app;
use app::App;

// Application Entrypoint
fn main() -> Result<(), Box<dyn Error>> {
    let mut app: App = App::new();

    let mut terminal = ratatui::init();
    let app_result = app.run(&mut terminal);
    ratatui::restore();

    app_result?;
    Ok(())
}

// Application Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_ok() {
        assert!(main().is_ok());
    }
}