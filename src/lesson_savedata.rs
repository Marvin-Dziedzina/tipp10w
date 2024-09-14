use std::fmt::Display;

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
}

impl Display for LessonSaveData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ ID: {} | Name: {} | Timestamp: {} | Duration: {} | Strokes: {} | Errors: {} ]",
            format!("{:<width$}", self.id, width = 4),
            format!("{:<width$}", self.name, width = 29),
            self.timestamp,
            format!("{:<width$}", self.duration, width = 5),
            format!("{:<width$}", self.strokes, width = 6),
            format!("{:<width$}", self.errors, width = 6),
        )
    }
}
