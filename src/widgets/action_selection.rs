use std::io;

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::Rect,
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{state::State, tipp10w::EventResult};

pub struct ActionSelection {
    ptr: usize,
}
impl ActionSelection {
    pub fn new() -> Self {
        Self { ptr: 0 }
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        let actions = vec![String::from("Append"), String::from("Delete")];

        let mut action_lines = Vec::new();
        for (i, action) in actions.iter().enumerate() {
            if i == self.ptr {
                action_lines.push(Line::from(
                    Span::from(action).fg(Color::Black).bg(Color::Yellow),
                ));
            } else {
                action_lines.push(Line::from(
                    Span::from(action).fg(Color::White).bg(Color::Black),
                ));
            };
        }

        let block = Block::default()
            .title("Actions")
            .borders(Borders::ALL)
            .border_type(BorderType::Thick);
        let paragraph = Paragraph::new(action_lines)
            .block(block)
            .wrap(Wrap { trim: false });

        f.render_widget(paragraph, area);
    }

    pub fn handle_events(&mut self, event: &Event) -> io::Result<EventResult> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Up => {
                        if self.ptr == 0 {
                            self.ptr = 1;
                        } else {
                            self.ptr -= 1;
                        };
                    }
                    KeyCode::Down => {
                        if self.ptr >= 1 {
                            self.ptr = 0;
                        } else {
                            self.ptr += 1;
                        };
                    }
                    KeyCode::Enter => match self.ptr {
                        0 => return Ok(EventResult::SubmitState(State::Append)),
                        1 => return Ok(EventResult::SubmitState(State::Delete)),
                        _ => {}
                    },
                    KeyCode::Esc => {
                        return Ok(EventResult::Exit);
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(EventResult::None)
    }
}
