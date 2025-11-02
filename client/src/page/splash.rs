/*

    Splash Page

    Griffin Dalby
    2025.10.30

    This will provide a splash page for the oxide client, displayed on startup.

*/

// Implement
use crate::app::{Page};


// Splash Page
#[derive(Default)]
pub struct SplashPage;

impl Page for SplashPage {
    fn render(&self, frame: &mut Frame) {
        let size = frame.size();
        let block = Block::default()
            .borders(border::ALL)
            .title("Oxide Client - Splash Page".stylize().bold());
        frame.render_widget(block, size);
    }

    fn handle_event(&mut self, event: &Event, app_state: &mut AppState) -> Result<()> {
        if let Event::Key(key) = event {
            if key.code == KeyCode::Enter {
                app_state.username = Some("griffin".to_string());
            }
        }
        Ok(())
    }
}