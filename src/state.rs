use crate::widgets::TextBox;

pub struct AppState {
    pub state: State,
    pub text_box: TextBox,
}
impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            state: State::Setup,
            text_box: TextBox::new(" Database Path "),
        }
    }
}

#[derive(Debug)]
pub enum State {
    Setup,
    Menu,
    Append,
    Delete,
}
