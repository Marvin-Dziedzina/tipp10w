use std::io;

use log::error;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use rusqlite::Connection;

use crate::{state::State, tipp10, tipp10w::EventResult};

use super::TextBox;

pub struct ParameterWidget {
    ptr: usize,
    // 1: lesson_id, 2: strokes, 3: errors, 4: timelen
    parameters: Vec<u64>,
}
impl ParameterWidget {
    pub fn new() -> Self {
        Self {
            ptr: 0,
            parameters: vec![18, 0, 0, 600, tipp10::get_timestamp()],
        }
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        let mut lines = Vec::new();
        for (i, v) in self.parameters.iter().enumerate() {
            let line: String;
            match i {
                0 => line = format!("ID: {}", v),
                1 => line = format!("Strokes: {}", v),
                2 => line = format!("Errors: {}", v),
                3 => line = format!("Timelen: {}", v),
                4 => line = format!("Timestamp: {}", v),
                _ => {
                    error!("Index out of bounds!");
                    panic!("Index out of bounds!");
                }
            };

            if self.ptr == i {
                lines.push(Line::from(
                    Span::from(line).fg(Color::Black).bg(Color::Yellow),
                ));
            } else {
                lines.push(Line::from(Span::from(line)));
            }
        }

        let block = Block::default()
            .title("Parameters")
            .borders(Borders::ALL)
            .border_type(BorderType::Thick);
        let paragraph = Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Left);

        f.render_widget(paragraph, area);
    }

    pub fn handle_events(
        &mut self,
        conn: &Connection,
        text_box: &mut TextBox,
        event: &Event,
    ) -> io::Result<EventResult> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Tab => {
                        if self.ptr != 0 {
                            self.ptr -= 1;
                        };

                        let buf = self.parameters[self.ptr].to_string();
                        text_box.set_buf(&buf);
                        text_box.set_ptr(buf.chars().count())?;

                        Ok(EventResult::None)
                    }
                    KeyCode::Enter => {
                        if self.ptr == self.parameters.len() {
                            match tipp10::append_lesson(
                                conn,
                                self.parameters[0] as u8,
                                self.parameters[1] as u32,
                                self.parameters[2] as u32,
                                self.parameters[3] as u32,
                                self.parameters[4],
                            ) {
                                Ok(_) => {
                                    self.reset();
                                    text_box.reset();
                                    return Ok(EventResult::SubmitState(State::Menu));
                                }
                                Err(e) => {
                                    error!("Could not append lesson to database! Error: {}", e);
                                    return Ok(EventResult::SubmitState(State::Menu));
                                }
                            };
                        } else {
                            self.parameters[self.ptr] =
                                match text_box.get_buffer_ref().parse::<u64>() {
                                    Ok(v) => v,
                                    Err(_) => {
                                        if self.ptr == 4 {
                                            tipp10::get_timestamp()
                                        } else {
                                            self.parameters[self.ptr]
                                        }
                                    }
                                };

                            self.ptr += 1;

                            if self.ptr <= self.parameters.len() - 1 {
                                let buf = self.parameters[self.ptr].to_string();
                                text_box.set_buf(&buf);
                                text_box.set_ptr(buf.chars().count())?;
                            };
                        };

                        Ok(EventResult::None)
                    }
                    KeyCode::Esc => {
                        self.reset();
                        text_box.reset();

                        Ok(EventResult::SubmitState(State::Menu))
                    }
                    _ => Ok(EventResult::None),
                }
            }
            _ => Ok(EventResult::None),
        }
    }

    pub fn reset(&mut self) {
        self.ptr = 0;
        self.parameters = vec![18, 0, 0, 600, tipp10::get_timestamp()];
    }
}
