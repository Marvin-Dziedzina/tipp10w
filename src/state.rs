use crate::widgets::{LessonsWidget, TextBox}; // Importing necessary widgets

// Type alias for ID to improve code readability
pub type ID = usize;

// Main application state struct
pub struct AppState {
    pub state: State,                  // Current state of the application
    pub text_box: TextBox,             // TextBox widget for user input
    pub lessons_widget: LessonsWidget, // LessonsWidget to display lessons
    pub error: String,                 // String to store error messages
}

impl AppState {
    /// Creates a new instance of AppState with default values
    pub fn new() -> Self {
        Self {
            state: State::Setup,                  // Initial state is Setup
            text_box: TextBox::new(None),         // Initialize TextBox with no max length
            lessons_widget: LessonsWidget::new(), // Initialize LessonsWidget
            error: String::new(),                 // Initialize error message as an empty string
        }
    }
}

// Enum representing the different states of the application
#[derive(Debug, PartialEq)]
pub enum State {
    Setup,          // Initial setup state
    Menu(SubState), // Menu state with a substate
}

// Enum representing the substates of the Menu state
#[derive(Debug, PartialEq)]
pub enum SubState {
    Edit(ID), // Edit substate with an ID
    None,     // No substate
}
