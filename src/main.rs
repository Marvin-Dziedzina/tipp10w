use std::io::{self, Write};

// Module declarations
pub mod events;
pub mod state;
mod tipp10;
mod tipp10w;
pub mod ui;
pub mod widgets;

use tipp10w::Tipp10W;

/// Enables bracketed paste mode in the terminal.
/// This helps the terminal application distinguish between user-typed input and pasted text.
fn enable_bracketed_paste() {
    print!("\x1b[?2004h");
    io::stdout().flush().unwrap();
}

/// Disables bracketed paste mode in the terminal.
fn disable_bracketed_paste() {
    print!("\x1b[?2004l");
    io::stdout().flush().unwrap();
}

fn main() -> io::Result<()> {
    env_logger::init();

    enable_bracketed_paste();

    let mut terminal: ratatui::Terminal<ratatui::prelude::CrosstermBackend<io::Stdout>> =
        ratatui::init();

    // Create a new instance of the application and run it.
    let app_result = Tipp10W::new().run(&mut terminal);

    ratatui::restore();

    disable_bracketed_paste();

    app_result
}
