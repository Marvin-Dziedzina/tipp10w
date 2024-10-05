use log::error;

use ratatui::{
    crossterm::event::Event,
    layout::Rect,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};
use rusqlite::Connection;

use crate::tipp10;

pub struct LessonsWidget {}
impl LessonsWidget {
    pub fn draw(f: &mut Frame, area: Rect, conn: &Connection) {
        let lessons = match tipp10::get_lessons(conn) {
            Ok(lessons) => lessons,
            Err(e) => {
                error!("Could not get lessons from database!");
                panic!("Could not get lessons from database: {}", e);
            }
        };

        let block = Block::default()
            .title("Lessons")
            .borders(Borders::ALL)
            .border_type(BorderType::Thick);

        let lines: Vec<Line> = lessons.iter().map(|lesson| lesson.get_line()).collect();
        let paragraph = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(block);

        f.render_widget(paragraph, area);
    }

    pub fn handle_events(event: &Event) {
        if let Event::Key(key_event) = event {
            todo!("LessonsWidget::handle_events")
        }
    }
}
