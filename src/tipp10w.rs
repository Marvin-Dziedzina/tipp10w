use std::{io, path::PathBuf};

// Module for handling event results and errors
mod event_result;

use ratatui::DefaultTerminal;
use rusqlite::Connection;

// Re-exporting EventResult and ResultError for use in other modules
pub use event_result::{EventResult, ResultError};

use crate::state::AppState;

/// Main application struct for Tipp10W
pub struct Tipp10W {
    pub app_state: AppState,      // Holds the state of the application
    pub conn: Option<Connection>, // Database connection to the SQLite database is Some if not in Setup state
}

impl Tipp10W {
    /// Creates a new instance of Tipp10W with default values
    pub fn new() -> Self {
        Self {
            app_state: AppState::new(),
            conn: None,
        }
    }

    /// Runs the main event loop of the application
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            // Draw the user interface
            self.draw_ui(terminal)?;

            // Handle events and check if the application should exit
            if EventResult::Exit == self.handle_events(terminal)? {
                break;
            };
        }

        Ok(())
    }

    /// Constructs the path to the database file
    pub fn get_path_to_db(path: &str) -> PathBuf {
        let mut path = PathBuf::from(path);
        path.extend(["portable", "tipp10v2.db"]); // Append the subdirectory and database file name

        path
    }
}
