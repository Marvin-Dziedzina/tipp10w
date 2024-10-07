use std::io;

use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::Paragraph;

use crate::tipp10w::{EventResult, ResultError};

/// A text box widget that allows the user to input text.
#[derive(Debug)]
pub struct TextBox {
    /// Points to the char the cursor is at. Insertion will happen before the pointer.
    ptr: usize,
    buf: String,
    max_len: Option<usize>,
}

impl TextBox {
    /// Creates a new instance of TextBox.
    pub fn new(max_len: Option<usize>) -> Self {
        Self {
            ptr: 0,
            buf: String::new(),
            max_len,
        }
    }

    /// Creates a new instance of TextBox with a preset value.
    pub fn with_preset(preset: &str, max_len: Option<usize>) -> Self {
        Self {
            ptr: 0,
            buf: preset.to_string(),
            max_len,
        }
    }

    pub fn draw(&self) -> Paragraph<'_> {
        let mut buf = self.buf.clone();
        // Add space to end so cursor can be at the end of the buffer
        buf.push(' ');

        // Create spans for each character in the buffer
        let mut spans: Vec<Span> = Vec::with_capacity(buf.chars().count());
        for (i, c) in buf.chars().enumerate() {
            if i == self.ptr {
                spans.push(
                    Span::from(c.to_string())
                        .style(Style::default())
                        .fg(Color::Black)
                        .bg(Color::White),
                );
            } else {
                spans.push(Span::from(c.to_string()).style(Style::default()));
            };
        }

        Paragraph::new(Line::from_iter(spans))
    }

    pub fn handle_events(&mut self, event: &Event) -> io::Result<EventResult> {
        let event_result: EventResult = match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Left => {
                        // Move the pointer to the left if it is not at the beginning
                        if self.ptr > 0 {
                            self.ptr -= 1;
                        };

                        EventResult::None(ResultError::None)
                    }
                    KeyCode::Right => {
                        // Move the pointer to the right if it is not at the end
                        if self.ptr < self.buf.chars().count() {
                            self.ptr += 1;
                        };

                        EventResult::None(ResultError::None)
                    }
                    KeyCode::Enter => EventResult::Submit, // Send a signal that the user has finished input
                    KeyCode::Backspace => {
                        // Remove the character before the pointer if it is not at the beginning
                        if self.ptr > 0 {
                            self.buf.remove(self.ptr - 1);
                            self.ptr -= 1;
                        };

                        EventResult::None(ResultError::None)
                    }
                    KeyCode::Delete => {
                        // Remove the character after the pointer if it is not at the end
                        if self.ptr < self.buf.chars().count() {
                            self.buf.remove(self.ptr);
                        };

                        EventResult::None(ResultError::None)
                    }
                    KeyCode::Char(c) => {
                        // Check for max length
                        if let Some(max_len) = self.max_len {
                            if self.buf.chars().count() >= max_len {
                                return Ok(EventResult::None(ResultError::MaxLenReached));
                            };
                        };

                        // Insert the character at the pointer
                        self.buf.insert(self.ptr, c);
                        self.ptr += 1;

                        EventResult::None(ResultError::None)
                    }
                    _ => EventResult::None(ResultError::None),
                }
            }
            Event::Paste(pasted) => {
                // Check for max length
                if let Some(max_len) = self.max_len {
                    if self.buf.chars().count() + pasted.chars().count() > max_len {
                        return Ok(EventResult::None(ResultError::MaxLenReached));
                    };
                };

                // Insert the pasted text at the pointer
                self.buf.insert_str(self.ptr, pasted);
                self.ptr += pasted.chars().count();

                EventResult::None(ResultError::None)
            }
            _ => EventResult::None(ResultError::None),
        };

        Ok(event_result)
    }

    /// Get the buffer of the text box.
    pub fn get_buffer(&self) -> String {
        self.buf.clone()
    }

    /// Get the buffer of the text box as a reference.
    pub fn get_buffer_ref(&self) -> &str {
        self.buf.as_str()
    }

    /// Set the buffer of the text box.
    pub fn set_buf(&mut self, buf: &str) {
        self.buf = buf.to_string();
    }

    /// Set the pointer of the text box.
    pub fn set_ptr(&mut self, ptr: usize) -> io::Result<()> {
        // Check if the pointer is out of bounds
        if ptr > self.buf.chars().count() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Pointer out of bounds!",
            ));
        };

        self.ptr = ptr;

        Ok(())
    }

    /// Set the max length of the text box.
    pub fn set_max_len(&mut self, max_len: Option<usize>) {
        self.max_len = max_len;
    }

    /// Reset the text box.
    pub fn reset(&mut self) {
        self.ptr = 0;
        self.buf.clear();
    }
}
#[cfg(test)]
mod tests {
    use ratatui::crossterm::event::{KeyEvent, KeyModifiers};

    use super::*;

    #[test]
    fn test_new_text_box() {
        let text_box = TextBox::new(Some(10));
        assert_eq!(text_box.ptr, 0);
        assert_eq!(text_box.buf, "");
        assert_eq!(text_box.max_len, Some(10));
    }

    #[test]
    fn test_with_preset() {
        let text_box = TextBox::with_preset("hello", Some(10));
        assert_eq!(text_box.ptr, 0);
        assert_eq!(text_box.buf, "hello");
        assert_eq!(text_box.max_len, Some(10));
    }

    #[test]
    fn test_handle_events_left_key() {
        let mut text_box = TextBox::with_preset("hello", None);
        text_box.set_ptr(3).unwrap();
        let event = Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
        text_box.handle_events(&event).unwrap();
        assert_eq!(text_box.ptr, 2);
    }

    #[test]
    fn test_handle_events_right_key() {
        let mut text_box = TextBox::with_preset("hello", None);
        text_box.set_ptr(3).unwrap();
        let event = Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        text_box.handle_events(&event).unwrap();
        assert_eq!(text_box.ptr, 4);
    }

    #[test]
    fn test_handle_events_backspace_key() {
        let mut text_box = TextBox::with_preset("hello", None);
        text_box.set_ptr(3).unwrap();
        let event = Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
        text_box.handle_events(&event).unwrap();
        assert_eq!(text_box.buf, "helo");
        assert_eq!(text_box.ptr, 2);
    }

    #[test]
    fn test_handle_events_delete_key() {
        let mut text_box = TextBox::with_preset("hello", None);
        text_box.set_ptr(3).unwrap();
        let event = Event::Key(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
        text_box.handle_events(&event).unwrap();
        assert_eq!(text_box.buf, "helo");
        assert_eq!(text_box.ptr, 3);
    }

    #[test]
    fn test_handle_events_char_key() {
        let mut text_box = TextBox::with_preset("hello", None);
        text_box.set_ptr(3).unwrap();
        let event = Event::Key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
        text_box.handle_events(&event).unwrap();
        assert_eq!(text_box.buf, "helxlo");
        assert_eq!(text_box.ptr, 4);
    }

    #[test]
    fn test_handle_events_paste() {
        let mut text_box = TextBox::with_preset("hello", None);
        text_box.set_ptr(3).unwrap();
        let event = Event::Paste("world".to_string());
        text_box.handle_events(&event).unwrap();
        assert_eq!(text_box.buf, "helworldlo");
        assert_eq!(text_box.ptr, 8);
    }

    #[test]
    fn test_set_ptr_out_of_bounds() {
        let mut text_box = TextBox::with_preset("hello", None);
        let result = text_box.set_ptr(10);
        assert!(result.is_err());
    }

    #[test]
    fn test_reset() {
        let mut text_box = TextBox::with_preset("hello", None);
        text_box.reset();
        assert_eq!(text_box.ptr, 0);
        assert_eq!(text_box.buf, "");
    }
}
