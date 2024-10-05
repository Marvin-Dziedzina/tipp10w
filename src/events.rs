use std::{io, path::PathBuf};

use log::error;
use ratatui::crossterm::event::{self, Event, KeyCode};
use rusqlite::Connection;

use crate::{
    state::State,
    tipp10w::{EventResult, Tipp10W},
    widgets::LessonsWidget,
};

impl Tipp10W {
    pub fn handle_events(&mut self) -> io::Result<EventResult> {
        let event = event::read()?;
        let result: EventResult = match &mut self.app_state.state {
            State::Setup => {
                let event_result = self.app_state.text_box.handle_events(&event)?;
                if event_result == EventResult::Submit {
                    let mut path = PathBuf::from(self.app_state.text_box.get_buffer_ref());
                    path.push("portable/tipp10v2.db");

                    self.conn = Some(match Connection::open(path) {
                        Ok(conn) => conn,
                        Err(e) => {
                            error!("Could not open database connection!");
                            panic!("Could not open database connection: {}", e);
                        }
                    });
                    self.app_state.state = State::Menu;
                    self.app_state.text_box.reset();
                    self.app_state.text_box.set_title("Value");
                };

                event_result
            }
            State::Menu => {
                if self.conn.is_some() {
                    LessonsWidget::handle_events(&event);

                    if let Event::Key(key_event) = event {
                        match key_event.code {
                            KeyCode::Esc => EventResult::Exit,
                            _ => EventResult::None,
                        }
                    } else {
                        EventResult::None
                    }
                } else {
                    self.app_state.state = State::Setup;
                    EventResult::None
                }
            }
            State::Append => EventResult::Exit,
            State::Delete => EventResult::Exit,
        };

        Ok(result)
    }
}
