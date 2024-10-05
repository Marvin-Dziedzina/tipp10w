use crate::widgets::{ActionSelection, ParameterWidget, TextBox};

pub struct AppState {
    pub state: State,
    pub text_box: TextBox,
    pub action_selection: ActionSelection,
    pub parameter_widget: ParameterWidget,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            state: State::Setup,
            text_box: TextBox::new(" Database Path ", true),
            action_selection: ActionSelection::new(),
            parameter_widget: ParameterWidget::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Setup,
    Menu,
    Append,
    Delete,
}
