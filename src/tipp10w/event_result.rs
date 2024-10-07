use std::fmt::Display;

use crate::state::{State, SubState};

/// Enum representing the different event results
#[derive(Debug, PartialEq)]
pub enum EventResult {
    Submit,
    SetState(State),
    SetSubState(SubState),
    None(ResultError),
    Exit,
}

/// Enum representing the different result errors
#[derive(Debug, PartialEq)]
pub enum ResultError {
    None,
    SQLite,
    Io,
    NoLessons,
    MaxLenReached,
    Timestamp,
    WrongInput,
    OutOfBounds,
    TextBoxError,
}
impl Display for ResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultError::None => write!(f, "No error occurred!"),
            ResultError::SQLite => write!(f, "SQLite error occurred!"),
            ResultError::Io => write!(f, "Io error occurred!"),
            ResultError::NoLessons => write!(f, "No lessons found!"),
            ResultError::MaxLenReached => write!(f, "Max length reached!"),
            ResultError::Timestamp => write!(f, "Timestamp error occurred!"),
            ResultError::WrongInput => write!(f, "Wrong input!"),
            ResultError::OutOfBounds => write!(f, "Out of bounds error occurred!"),
            ResultError::TextBoxError => write!(f, "TextBox error occurred!"),
        }
    }
}
