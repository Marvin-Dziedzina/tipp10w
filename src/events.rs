use std::{io, path::PathBuf};

use log::error;
use ratatui::crossterm::event;
use rusqlite::Connection;

use crate::{
    state::State,
    tipp10w::{EventResult, Tipp10W}, widgets::LessonsWidget,
};

impl Tipp10W {
    pub fn handle_events(&mut self) -> io::Result<EventResult> {
        let event = event::read()?;
        let result: EventResult = match &mut self.app_state.state {
            State::Setup => {
                self.app_state.text_box.select();
                let event_result = self.app_state.text_box.handle_events(&event)?;
                if event_result == EventResult::Submit {
                    let mut path = PathBuf::from(self.app_state.text_box.get_buffer_ref());
                    path.extend(["portable", "tipp10v2.db"]);

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
                    self.app_state.text_box.deselect();
                };

                event_result
            }
            State::Menu => {
                if self.conn.is_some() {
                    let event_result = self.app_state.action_selection.handle_events(&event)?;

                    match event_result {
                        EventResult::SubmitState(state) => {
                            self.app_state.state = state;
                            self.app_state.text_box.select();
                            LessonsWidget::h

                            EventResult::None
                        }
                        EventResult::Exit => return Ok(EventResult::Exit),
                        _ => EventResult::None,
                    }
                } else {
                    self.app_state.state = State::Setup;
                    EventResult::None
                }
            }
            State::Append => {
                if let Some(conn) = &self.conn {
                    let event_result = self.app_state.parameter_widget.handle_events(
                        conn,
                        &mut self.app_state.text_box,
                        &event,
                    )?;
                    self.app_state.text_box.handle_events(&event)?;

                    match event_result {
                        EventResult::SubmitState(state) => {
                            self.app_state.state = state;
                            EventResult::None
                        }
                        EventResult::Exit => EventResult::Exit,
                        _ => EventResult::None,
                    }
                } else {
                    self.app_state.state = State::Setup;
                    EventResult::None
                }
            }
            State::Delete => {
                todo!("Delete state not implemented yet!")
            }
        };

        Ok(result)
    }
}
