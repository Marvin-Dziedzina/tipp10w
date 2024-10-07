use std::fmt::Display;

use ratatui::{
    style::{Color, Stylize},
    text::Span,
};

use super::LessonSelection;

/// A struct that represents a lesson.
#[derive(Debug, Clone)]
pub struct Lesson {
    pub id: usize,
    pub lesson_id: LessonSelection,
    pub timestamp: u64,
    pub duration: usize,
    pub strokes: usize,
    pub errors: usize,
}
impl Lesson {
    /// Creates a new instance of Lesson.
    pub fn new(
        id: usize,
        lesson_selection: LessonSelection,
        timestamp: u64,
        duration: usize,
        strokes: usize,
        errors: usize,
    ) -> Self {
        Self {
            id,
            lesson_id: lesson_selection,
            timestamp,
            duration,
            strokes,
            errors,
        }
    }

    /// Get a line of text representing the lesson.
    pub fn get_line(&self) -> [Span; 19] {
        [
            Span::from("[ "),
            Span::from("ID: "),
            Span::from(format!("{:<4}", self.id)).fg(Color::Yellow),
            Span::from(" | "),
            Span::from("Name: "),
            Span::from(format!("{:<29}", self.lesson_id.get_lesson_name())).fg(Color::Yellow),
            Span::from(" | "),
            Span::from("Timestamp: "),
            Span::from(format!("{:<14}", self.timestamp)).fg(Color::Yellow),
            Span::from(" | "),
            Span::from("Duration: "),
            Span::from(format!("{:<4}", self.duration)).fg(Color::Yellow),
            Span::from(" | "),
            Span::from("Strokes: "),
            Span::from(format!("{:<6}", self.strokes)).fg(Color::Yellow),
            Span::from(" | "),
            Span::from("Errors: "),
            Span::from(format!("{:<6}", self.errors)).fg(Color::Yellow),
            Span::from(" ]"),
        ]
    }
}

impl Display for Lesson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ ID: {} | Name: {} | Timestamp: {} | Duration: {} | Strokes: {} | Errors: {} ]",
            format!("{:<4}", self.id),
            format!("{:<29}", self.lesson_id.get_lesson_name()),
            self.timestamp,
            format!("{:<4}", self.duration),
            format!("{:<6}", self.strokes),
            format!("{:<6}", self.errors),
        )
    }
}
