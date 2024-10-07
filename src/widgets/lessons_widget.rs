use log::error;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::Rect,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use rusqlite::Connection;

use crate::{
    state::SubState,
    tipp10,
    tipp10w::{EventResult, ResultError},
};

use super::{LessonWidget, TextBox};

// This widget is responsible for rendering and handling events for the lessons
pub struct LessonsWidget {
    pub ptr: usize,
    pub lessons: Vec<LessonWidget>,
}
impl Default for LessonsWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl LessonsWidget {
    pub fn new() -> Self {
        Self {
            ptr: 0,
            lessons: Vec::new(),
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect, sub_state: &SubState, text_box: &mut TextBox) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title(" Lessons ");

        // Subtract 2 from the height to account for the border
        let height = area.height as usize;
        let lines_to_subtract = if 0 < self.ptr as isize + 3 - height as isize {
            self.ptr + 3 - height
        } else {
            0
        };

        // Render each lesson
        let mut lines = Vec::new();
        for (i, lesson) in self.lessons.iter().enumerate() {
            // Check if the lesson is selected
            let selected = i == self.ptr;

            // Check if the lesson is being edited
            let is_editing = match sub_state {
                SubState::Edit(id) => *id == lesson.lesson.id,
                _ => false,
            };

            lines.push(lesson.draw(selected, is_editing, text_box));
        }

        // Remove the number of lines calculated in lines_to_subtract
        if lines_to_subtract > 0 {
            lines.drain(0..lines_to_subtract);
        };

        f.render_widget(Paragraph::new(lines).block(block), area);
    }

    pub fn handle_events(
        &mut self,
        event: Event,
        sub_state: &SubState,
        conn: &Connection,
        text_box: &mut TextBox,
    ) -> EventResult {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => match sub_state {
                SubState::None => match key_event.code {
                    KeyCode::Up => {
                        // Check if there are any lessons and move the pointer up
                        if self.ptr > 0 {
                            self.ptr -= 1;
                        };

                        EventResult::None(ResultError::None)
                    }
                    KeyCode::Down => {
                        if self.lessons.is_empty() {
                            // Check if there are any lessons
                            return EventResult::None(ResultError::None);
                        };

                        // If there are lessons and the pointer is not at the last lesson, move the pointer down
                        if self.ptr < self.lessons.len() - 1 {
                            self.ptr += 1;
                        };

                        EventResult::None(ResultError::None)
                    }
                    KeyCode::Enter => {
                        if self.lessons.is_empty() {
                            return EventResult::None(ResultError::NoLessons);
                        };

                        EventResult::SetSubState(SubState::Edit(self.lessons[self.ptr].lesson.id))
                    }
                    KeyCode::Delete => {
                        // Check if there are any lessons
                        if self.lessons.is_empty() {
                            return EventResult::None(ResultError::NoLessons);
                        };

                        // Delete the lesson from the database
                        match tipp10::delete_lesson(conn, self.lessons[self.ptr].lesson.id) {
                            Ok(_) => (),
                            Err(e) => {
                                error!("Could not delete lesson from database! Error: {}", e);
                                return EventResult::None(ResultError::SQLite);
                            }
                        };

                        self.update_lessons(conn);

                        if self.ptr >= self.lessons.len() && !self.lessons.is_empty() {
                            self.ptr = self.lessons.len() - 1;
                        };

                        EventResult::None(ResultError::None)
                    }
                    KeyCode::Char('n') => {
                        let id = match tipp10::append_lesson(
                            conn,
                            18,
                            0,
                            0,
                            600,
                            tipp10::get_timestamp(),
                        ) {
                            Ok(lesson) => lesson,
                            Err(e) => {
                                error!("Could not append lesson to database! Error: {}", e);
                                return EventResult::None(ResultError::SQLite);
                            }
                        };

                        self.update_lessons_and_move_ptr_to_last(conn);
                        EventResult::SetSubState(SubState::Edit(id))
                    }
                    KeyCode::Char('u') => {
                        self.update_lessons(conn);

                        if self.lessons.is_empty() {
                            return EventResult::None(ResultError::NoLessons);
                        };

                        if self.ptr > self.lessons.len() - 1 {
                            self.move_ptr_to_last();
                        };

                        EventResult::None(ResultError::None)
                    }
                    KeyCode::Esc => EventResult::Exit,
                    _ => EventResult::None(ResultError::None),
                },
                SubState::Edit(id) => {
                    // Handle events for the lesson being edited
                    for lesson in &mut self.lessons {
                        if lesson.lesson.id == *id {
                            return lesson.handle_events(event, conn, text_box);
                        };
                    }

                    EventResult::SetSubState(SubState::None)
                }
            },
            _ => EventResult::None(ResultError::None),
        }
    }

    /// Get the lessons from the database and return them as a vector of LessonWidget
    fn get_lessons(&mut self, conn: &Connection) -> Vec<LessonWidget> {
        let lessons_save_data = tipp10::get_lessons(conn).expect("Could not get lessons!");
        lessons_save_data
            .iter()
            .map(|lesson| LessonWidget::new(lesson.clone()))
            .collect()
    }

    /// Update the lessons from the database
    pub fn update_lessons(&mut self, conn: &Connection) {
        self.lessons = self.get_lessons(conn);
    }

    /// Move the pointer to the last lesson
    pub fn move_ptr_to_last(&mut self) {
        if !self.lessons.is_empty() {
            self.ptr = self.lessons.len() - 1;
        };
    }

    /// Update the lessons from the database and move the pointer to the last lesson
    pub fn update_lessons_and_move_ptr_to_last(&mut self, conn: &Connection) {
        self.update_lessons(conn);
        self.move_ptr_to_last();
    }
}
