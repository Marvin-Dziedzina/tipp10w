use std::fmt::Display;

use ratatui::{
    style::{Color, Stylize},
    text::{Line, Span},
};

#[derive(Debug)]
pub struct LessonSaveData {
    id: u32,
    name: String,
    timestamp: String,
    duration: u32,
    strokes: u32,
    errors: u32,
}
impl LessonSaveData {
    pub fn new(
        id: u32,
        name: String,
        timestamp: String,
        duration: u32,
        strokes: u32,
        errors: u32,
    ) -> Self {
        Self {
            id,
            name,
            timestamp,
            duration,
            strokes,
            errors,
        }
    }

    pub fn get_line(&self) -> Line {
        let spans = vec![
            Span::from("[ ID: "),
            Span::from(format!("{:<4}", self.id)).fg(Color::Yellow),
            Span::from(" | Name: "),
            Span::from(format!("{:<29}", self.name)).fg(Color::Yellow),
            Span::from(" | Timestamp: "),
            Span::from(format!("{:<14}", self.timestamp.to_string())).fg(Color::Yellow),
            Span::from(" | Duration: "),
            Span::from(format!("{:<4}", self.duration)).fg(Color::Yellow),
            Span::from(" | Strokes: "),
            Span::from(format!("{:<6}", self.strokes)).fg(Color::Yellow),
            Span::from(" | Errors: "),
            Span::from(format!("{:<6}", self.errors)).fg(Color::Yellow),
            Span::from(" ]"),
        ];

        Line::from(spans)
    }
}

impl Display for LessonSaveData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ ID: {} | Name: {} | Timestamp: {} | Duration: {} | Strokes: {} | Errors: {} ]",
            format!("{:<4}", self.id),
            format!("{:<29}", self.name),
            self.timestamp,
            format!("{:<4}", self.duration),
            format!("{:<6}", self.strokes),
            format!("{:<6}", self.errors),
        )
    }
}
