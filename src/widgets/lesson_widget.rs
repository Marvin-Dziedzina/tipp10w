use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::Alignment,
    style::{Color, Stylize},
    text::Line,
};
use rusqlite::Connection;

use crate::{
    state::SubState,
    tipp10::{self, Lesson, LessonSelection},
    tipp10w::{EventResult, ResultError},
};

use super::TextBox;

/// A widget that represents a lesson.
pub struct LessonWidget {
    pub ptr: usize,
    pub lesson: Lesson,
}
impl LessonWidget {
    /// Creates a new instance of LessonWidget.
    pub fn new(lesson: Lesson) -> Self {
        Self { lesson, ptr: 0 }
    }

    pub fn draw(&self, selected: bool, is_editing: bool, text_box: &mut TextBox) -> Line<'_> {
        if is_editing {
            let mut lines = self.lesson.get_line();
            match self.ptr {
                0 => {
                    lines[5] = lines[5]
                        .clone()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .content(format!("{:<29}", text_box.get_buffer()));
                }
                1 => {
                    lines[8] = lines[8]
                        .clone()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .content(format!("{:<14}", text_box.get_buffer()));
                }
                2 => {
                    lines[11] = lines[11]
                        .clone()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .content(format!("{:<4}", text_box.get_buffer()));
                }
                3 => {
                    lines[14] = lines[14]
                        .clone()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .content(format!("{:<6}", text_box.get_buffer()));
                }
                4 => {
                    lines[17] = lines[17]
                        .clone()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .content(format!("{:<6}", text_box.get_buffer()));
                }
                _ => (),
            };

            Line::from_iter(lines).alignment(Alignment::Center)
        } else {
            if selected {
                Line::from_iter(self.lesson.get_line())
                    .bg(Color::DarkGray)
                    .fg(Color::Black)
                    .alignment(Alignment::Center)
            } else {
                Line::from_iter(self.lesson.get_line()).alignment(Alignment::Center)
            }
        }
    }

    pub fn handle_events(
        &mut self,
        event: Event,
        conn: &Connection,
        text_box: &mut TextBox,
    ) -> EventResult {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Enter => {
                        // Append the lesson to the database if the pointer is at the end
                        if self.ptr == 4 {
                            // Change the errors
                            self.lesson.errors = if text_box.get_buffer_ref().trim().is_empty() {
                                self.lesson.errors
                            } else {
                                let errors = match text_box.get_buffer_ref().parse::<usize>() {
                                    Ok(errors) => errors,
                                    Err(_) => {
                                        return EventResult::None(ResultError::WrongInput);
                                    }
                                };

                                text_box.reset();

                                errors
                            };

                            self.ptr = 0;

                            match tipp10::update_lesson(
                                conn,
                                self.lesson.id,
                                self.lesson.lesson_id.get_lesson_id(),
                                self.lesson.strokes,
                                self.lesson.errors,
                                self.lesson.duration,
                                self.lesson.timestamp,
                            ) {
                                Ok(_) => {
                                    return EventResult::SetSubState(SubState::None);
                                }
                                Err(_) => {
                                    return EventResult::None(ResultError::SQLite);
                                }
                            }
                        } else {
                            match self.ptr {
                                0 => {
                                    // Change the lesson id
                                    self.lesson.lesson_id =
                                        if text_box.get_buffer_ref().trim().is_empty() {
                                            self.lesson.lesson_id.clone()
                                        } else {
                                            let lession_selection = LessonSelection::from_lesson_id(
                                                match text_box.get_buffer_ref().parse::<u8>() {
                                                    Ok(id) => id,
                                                    Err(_) => {
                                                        return EventResult::None(
                                                            ResultError::WrongInput,
                                                        )
                                                    }
                                                },
                                            );

                                            text_box.reset();

                                            lession_selection
                                        };

                                    EventResult::None(ResultError::None)
                                }
                                1 => {
                                    // Change the timestamp
                                    self.lesson.timestamp =
                                        if text_box.get_buffer_ref().trim().is_empty() {
                                            tipp10::get_timestamp()
                                        } else {
                                            let timestamp =
                                                match text_box.get_buffer_ref().parse::<u64>() {
                                                    Ok(timestamp) => timestamp,
                                                    Err(_) => {
                                                        return EventResult::None(
                                                            ResultError::WrongInput,
                                                        );
                                                    }
                                                };

                                            text_box.reset();

                                            timestamp
                                        };

                                    EventResult::None(ResultError::None)
                                }
                                2 => {
                                    // Change the duration
                                    self.lesson.duration =
                                        if text_box.get_buffer_ref().trim().is_empty() {
                                            self.lesson.duration
                                        } else {
                                            let duration =
                                                match text_box.get_buffer_ref().parse::<usize>() {
                                                    Ok(duration) => duration,
                                                    Err(_) => {
                                                        return EventResult::None(
                                                            ResultError::WrongInput,
                                                        );
                                                    }
                                                };

                                            text_box.reset();

                                            duration
                                        };

                                    EventResult::None(ResultError::None)
                                }
                                3 => {
                                    // Change the strokes
                                    self.lesson.strokes =
                                        if text_box.get_buffer_ref().trim().is_empty() {
                                            self.lesson.strokes
                                        } else {
                                            let strokes =
                                                match text_box.get_buffer_ref().parse::<usize>() {
                                                    Ok(strokes) => strokes,
                                                    Err(_) => {
                                                        return EventResult::None(
                                                            ResultError::WrongInput,
                                                        );
                                                    }
                                                };

                                            text_box.reset();

                                            strokes
                                        };

                                    EventResult::None(ResultError::None)
                                }
                                _ => EventResult::None(ResultError::OutOfBounds),
                            };

                            // Move the pointer to the right if it is not at the end
                            self.ptr += 1;
                            self.set_max_lenght(text_box);
                        };

                        EventResult::None(ResultError::None)
                    }
                    KeyCode::Tab => {
                        // Move the pointer to the left if it is not at the beginning
                        if self.ptr > 0 {
                            self.ptr -= 1;
                        };
                        self.set_max_lenght(text_box);

                        text_box.reset();

                        EventResult::None(ResultError::None)
                    }
                    _ => match text_box.handle_events(&event) {
                        Ok(_) => EventResult::None(ResultError::None),
                        Err(_) => EventResult::None(ResultError::TextBoxError),
                    },
                }
            }
            _ => EventResult::None(ResultError::None),
        }
    }

    fn set_max_lenght(&self, text_box: &mut TextBox) {
        match self.ptr {
            0 => {
                text_box.set_max_len(Some(2));
            }
            1 => {
                text_box.set_max_len(Some(14));
            }
            2 => {
                text_box.set_max_len(Some(4));
            }
            3 => {
                text_box.set_max_len(Some(6));
            }
            4 => {
                text_box.set_max_len(Some(6));
            }
            _ => (),
        };
    }
}
