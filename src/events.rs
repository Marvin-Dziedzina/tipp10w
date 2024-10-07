use std::io;

use log::warn;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::Rect,
    prelude::CrosstermBackend,
    Terminal,
};
use rusqlite::Connection;

use crate::{
    state::{State, SubState},
    tipp10w::{EventResult, ResultError, Tipp10W},
};

impl Tipp10W {
    pub fn handle_events(
        &mut self,
        f: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> io::Result<EventResult> {
        // Read the next event from the terminal
        let event = event::read()?;
        match event {
            Event::Resize(width, height) => {
                // Update the terminal size
                f.resize(Rect::new(0, 0, width, height))?;
            }
            _ => (),
        }

        let result: EventResult = match &mut self.app_state.state {
            State::Setup => {
                match event {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        match key_event.code {
                            KeyCode::Esc => return Ok(EventResult::Exit),
                            _ => (),
                        }
                    }
                    _ => (),
                };

                let event_result = self.app_state.text_box.handle_events(&event)?;
                match event_result {
                    EventResult::Submit => {
                        // Open a connection to the database using the path from the TextBox buffer
                        self.conn = Some(
                            match Connection::open(Tipp10W::get_path_to_db(
                                self.app_state.text_box.get_buffer_ref(),
                            )) {
                                Ok(conn) => conn,
                                Err(_) => {
                                    // Return the result error
                                    self.app_state.text_box.reset();

                                    warn!("Failed to open connection to database!");

                                    return Ok(EventResult::None(ResultError::SQLite));
                                }
                            },
                        );

                        // Change the application state to Menu with no substate
                        self.app_state.state = State::Menu(SubState::None);

                        // If the connection is successfully established, update lessons and move pointer to the last lesson
                        if let Some(conn) = &self.conn {
                            self.app_state
                                .lessons_widget
                                .update_lessons_and_move_ptr_to_last(conn);

                            self.app_state.text_box.reset();
                        };

                        EventResult::None(ResultError::None)
                    }
                    _ => event_result, // Return the original event result for other cases
                }
            }
            State::Menu(sub_state) => {
                if let Some(conn) = &self.conn {
                    // Handle events for the LessonsWidget
                    match self.app_state.lessons_widget.handle_events(
                        event,
                        sub_state,
                        conn,
                        &mut self.app_state.text_box,
                    ) {
                        EventResult::SetState(state) => {
                            // Change the application state
                            self.app_state.state = state;
                            EventResult::None(ResultError::None)
                        }
                        EventResult::SetSubState(sub_state) => {
                            // Change the substate of the Menu state
                            self.app_state.state = State::Menu(sub_state);
                            EventResult::None(ResultError::None)
                        }
                        EventResult::None(result_error) => match result_error {
                            ResultError::None => {
                                // Clear any existing error message
                                self.app_state.error = String::new();
                                EventResult::None(ResultError::None)
                            }
                            result_error => {
                                // Set the error message and return the result error
                                let e = result_error.to_string();
                                self.app_state.error = e;

                                EventResult::None(result_error)
                            }
                        },
                        event_result => event_result, // Return the original event result for other cases
                    }
                } else {
                    // If the connection is None, return to the Setup state
                    self.conn = None;
                    EventResult::SetState(State::Setup)
                }
            }
        };

        Ok(result)
    }
}
