use std::io;

use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use crate::tipp10w::EventResult;

#[derive(Debug)]
pub struct TextBox {
    is_selected: bool,
    title: String,
    /// Points to the char the cursor is at. Insertion will happen before the pointer.
    ptr: usize,
    buf: String,
}

impl TextBox {
    pub fn new(title: &str, is_selected: bool) -> Self {
        Self {
            is_selected: is_selected,
            title: title.to_string(),
            ptr: 0,
            buf: String::new(),
        }
    }

    pub fn with_preset(title: &str, preset: &str) -> Self {
        Self {
            is_selected: false,
            title: title.to_string(),
            ptr: 0,
            buf: preset.to_string(),
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let title = Title::from(self.title.as_str()).alignment(Alignment::Left);
        let block = Block::new()
            .border_type(BorderType::Thick)
            .title(title)
            .borders(Borders::ALL);

        let mut text_buf = self.buf.clone();
        text_buf.push(' ');

        let styled_text = if self.is_selected {
            let max_line_length = block.inner(area).width as usize;
            let chars_to_truncate = self.ptr as isize - max_line_length as isize + 3;
            if self.ptr >= max_line_length - 3 {
                text_buf = text_buf.chars().skip(chars_to_truncate as usize).collect();
            };

            let ptr = if chars_to_truncate > 0 {
                max_line_length - 3
            } else {
                self.ptr
            };

            let mut styled_text = vec![];
            for (i, c) in text_buf.chars().enumerate() {
                // Change the color of the character at the cursor position
                let styled_char = if i == ptr {
                    Span::styled(
                        c.to_string(),
                        Style::default().fg(Color::Black).bg(Color::White),
                    )
                } else {
                    Span::styled(
                        c.to_string(),
                        Style::default().fg(Color::White).bg(Color::Black),
                    )
                };

                styled_text.push(styled_char);
            }

            styled_text
        } else {
            let mut styled_text = Vec::new();
            styled_text.push(Span::from(text_buf).fg(Color::White).bg(Color::Black));

            styled_text
        };

        let line = Line::from(styled_text);
        let paragraph = Paragraph::new(line).block(block);

        f.render_widget(paragraph, area);
    }

    pub fn handle_events(&mut self, event: &Event) -> io::Result<EventResult> {
        if !self.is_selected {
            return Ok(EventResult::None);
        };

        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Esc => {
                        return Ok(EventResult::Exit);
                    }
                    KeyCode::Enter => {
                        return Ok(EventResult::Submit);
                    }
                    KeyCode::Backspace => {
                        if self.ptr != 0 {
                            self.buf.remove(self.ptr - 1);
                            self.ptr -= 1;
                        };
                    }
                    KeyCode::Delete => {
                        if self.ptr != self.buf.chars().count() {
                            self.buf.remove(self.ptr);
                        };
                    }
                    KeyCode::Left => {
                        if self.ptr != 0 {
                            self.ptr -= 1;
                        };
                    }
                    KeyCode::Right => {
                        if self.ptr != self.buf.chars().count() {
                            self.ptr += 1;
                        };
                    }

                    KeyCode::Char(c) => {
                        self.buf.insert(self.ptr, c);
                        self.ptr += 1;
                    }
                    _ => {}
                };
            }
            _ => {}
        };

        if let Event::Paste(pasted) = event {
            self.buf.insert_str(self.ptr, pasted.as_str());
            self.ptr += pasted.chars().count();
        };

        Ok(EventResult::None)
    }

    pub fn get_buffer(self) -> String {
        self.buf
    }

    pub fn get_buffer_ref(&self) -> &str {
        self.buf.as_str()
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn set_buf(&mut self, buf: &str) {
        self.buf = buf.to_string();
    }

    pub fn set_ptr(&mut self, ptr: usize) -> io::Result<()> {
        if ptr > self.buf.chars().count() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Pointer out of bounds!",
            ));
        };

        self.ptr = ptr;

        Ok(())
    }

    pub fn reset(&mut self) {
        self.ptr = 0;
        self.buf.clear();
    }

    pub fn select(&mut self) {
        self.is_selected = true;
    }

    pub fn deselect(&mut self) {
        self.is_selected = false;
    }

    pub fn toggle(&mut self) {
        self.is_selected = !self.is_selected;
    }
}
