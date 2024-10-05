use std::io;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    DefaultTerminal,
};

use crate::{state::State, tipp10w::Tipp10W, widgets::LessonsWidget};

impl Tipp10W {
    pub fn draw_ui(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|f| match &mut self.app_state.state {
            State::Setup => {
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(3), Constraint::Length(3)])
                    .split(f.area());

                self.app_state.text_box.render(f, layout[1]);
            }
            State::Menu => {
                if let Some(conn) = &self.conn {
                    let layout_vertical = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Min(3), Constraint::Length(3)])
                        .split(f.area());
                    let layout_horizontal = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Min(3), Constraint::Length(25)])
                        .split(layout_vertical[0]);

                    LessonsWidget::render(f, layout_horizontal[0], conn);
                    self.app_state
                        .action_selection
                        .render(f, layout_horizontal[1]);
                } else {
                    self.app_state.state = State::Setup;
                };
            }
            State::Append => {
                if let Some(conn) = &self.conn {
                    let layout_vertical = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Min(3), Constraint::Length(3)])
                        .split(f.area());
                    let layout_horizontal = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Min(3), Constraint::Length(25)])
                        .split(layout_vertical[0]);

                    self.app_state
                        .parameter_widget
                        .render(f, layout_horizontal[1]);
                    LessonsWidget::render(f, layout_horizontal[0], conn);
                    self.app_state.text_box.render(f, layout_vertical[1]);
                };
            }
            State::Delete => {
                todo!("Delete state not implemented yet!")
            }
        })?;

        Ok(())
    }
}
