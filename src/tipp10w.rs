use std::io;

mod event_result;

use ratatui::DefaultTerminal;
use rusqlite::Connection;

pub use event_result::EventResult;

use crate::state::AppState;

pub struct Tipp10W {
    pub app_state: AppState,
    pub conn: Option<Connection>,
}
impl Tipp10W {
    pub fn new() -> Self {
        Self {
            app_state: AppState::new(),
            conn: None,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            self.draw_ui(terminal)?;

            if EventResult::Exit == self.handle_events()? {
                break;
            };
        }

        Ok(())
    }
}
