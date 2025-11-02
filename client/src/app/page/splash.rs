/*

    Splash Page

    Griffin Dalby
    2025.10.30

    This will provide a splash page for the oxide client, displayed on startup.

*/

// Implement
use crate::app::page::{Page};
use crate::app::{AppState};

use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Layout, Offset};
use ratatui::text;
use ratatui::{
    buffer::Buffer,
    layout::{Rect, Constraint, Flex},
    style::{Modifier, Style, Color},
    text::{Line, Text, Span},
    widgets::{Block, Paragraph, Widget, Borders},
    Frame, symbols
};

// Functions
fn center_rect(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

// Splash Page
#[derive(Debug, Default)]
pub struct SplashPage;

impl Widget for &SplashPage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Border & Splash Text
        let block = Block::default()
            .borders(Borders::ALL);

        let splash_text = Text::from(vec![
            Line::from(vec![Span::styled(r" ______     __  __     __     _____     ______    ", Style::default().fg(Color::Rgb(254, 254, 254))).into()]),
            Line::from(vec![Span::styled(r"/\  __ \   /\_\_\_\   /\ \   /\  __-.  /\  ___\   ", Style::default().fg(Color::Rgb(184, 214, 227))).into()]),
            Line::from(vec![Span::styled(r"\ \ \/\ \  \/_/\_\/_  \ \ \  \ \ \/\ \ \ \  __\   ", Style::default().fg(Color::Rgb(96, 164, 199))).into()]),
            Line::from(vec![Span::styled(r" \ \_____\   /\_\/\_\  \ \_\  \ \____-  \ \_____\ ", Style::default().fg(Color::Rgb(68, 109, 143))).into()]),
            Line::from(vec![Span::styled(r"  \/_____/   \/_/\/_/   \/_/   \/____/   \/_____/ ", Style::default().fg(Color::Rgb(47, 85, 101))).into()]),
            Line::from(vec![r"".into()]),

            Line::from(vec![
                Span::styled("Made with ", Style::default().fg(Color::Rgb(96, 164, 199))).into(),
                Span::styled("♥", Style::default().fg(Color::Red)).into(),
                Span::styled(" by Griffin Dalby", Style::default().fg(Color::Rgb(96, 164, 199))).into(),
            ])
        ]);

        Paragraph::new(splash_text)
            .centered()
            .block(block)
            .render(area, buf);

        // Logs
        let log_block = Block::new()
            .title("Logs")
            .title_style(Style::new().fg(Color::Rgb(184, 214, 227)))
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED);
        
        let c_area = center_rect(area, Constraint::Percentage(65), Constraint::Percentage(65));
        let offset_res = c_area.offset(Offset { x: 0, y: 4 });
        log_block.render(offset_res, buf);
    }
}

impl Page for SplashPage {
    fn id(&self) -> &'static str { "splash" }
    fn title(&self) -> &'static str { "Splash Page" }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(&*self, frame.area());
    }

    fn handle_event(&mut self, event: &Event, app_state: &mut AppState) -> Result<(), Box<dyn std::error::Error>> {
        if let Event::Key(key) = event {
            if key.code == KeyCode::Enter {
                app_state.username = Some("griffin".to_string());
            }
        }
        Ok(())
    }
}