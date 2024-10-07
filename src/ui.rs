use std::io;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders},
    DefaultTerminal,
};

use crate::{state::State, tipp10w::Tipp10W};

impl Tipp10W {
    pub fn draw_ui(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|f| match &mut self.app_state.state {
            State::Setup => {
                // Create a vertical layout with 3 chunks
                let chunks_vertical = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Min(0),
                            Constraint::Length(3),
                            Constraint::Min(0),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());
                // Create a horizontal layout with 3 chunks
                let chunks_horizontal = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Min(0),
                        Constraint::Length(60),
                        Constraint::Min(0),
                    ])
                    .split(chunks_vertical[1]);

                // Render the text box in the center chunk of the horizontal layout
                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .title(" Tipp10 Path ");
                f.render_widget(
                    self.app_state.text_box.draw().block(block),
                    chunks_horizontal[1],
                );
            }
            State::Menu(sub_state) => {
                if self.conn.is_some() {
                    // Create a vertical layout with 2 chunks
                    let chunks_vertical = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Min(3), Constraint::Length(1)])
                        .split(f.area());
                    // Create a horizontal layout with 3 chunks
                    let status_bar = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([
                            Constraint::Fill(1),
                            Constraint::Length(1),
                            Constraint::Fill(1),
                        ])
                        .split(chunks_vertical[1]);

                    // Render the lessons widget in the top chunk of the vertical layout
                    self.app_state.lessons_widget.render(
                        f,
                        chunks_vertical[0],
                        sub_state,
                        &mut self.app_state.text_box,
                    );

                    // Create the help line
                    let help = Line::from_iter([
                        Span::from(" Up").fg(Color::Yellow),
                        Span::from(" | "),
                        Span::from("Down").fg(Color::Yellow),
                        Span::from(" | "),
                        Span::from("n").fg(Color::Yellow),
                        Span::from(": new | "),
                        Span::from("Enter").fg(Color::Yellow),
                        Span::from(": Edit | "),
                        Span::from("Del").fg(Color::Yellow),
                        Span::from(": Delete | "),
                        Span::from("u").fg(Color::Yellow),
                        Span::from(": update "),
                    ])
                    .alignment(Alignment::Center);

                    // Render the help line in the left chunk of the status bar
                    f.render_widget(help, status_bar[0]);

                    // Render a vertical separator in the middle chunk of the status bar
                    f.render_widget(Line::from("┃"), status_bar[1]);

                    // Render the error message in the right chunk of the status bar
                    f.render_widget(
                        Line::from_iter([
                            Span::from(" "),
                            Span::from(self.app_state.error.as_str()).fg(Color::Red),
                        ])
                        .alignment(Alignment::Center),
                        status_bar[2],
                    );
                }
            }
        })?;

        Ok(())
    }
}
