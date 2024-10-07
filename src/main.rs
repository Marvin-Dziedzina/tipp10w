use std::{
    fs::File,
    io::{self, Write},
};

// Module declarations
pub mod events;
pub mod state;
mod tipp10;
mod tipp10w;
pub mod ui;
pub mod widgets;

use env_logger::Builder;
use log::info;
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

/// Initializes the logger for the application.
#[cfg(debug_assertions)]
fn init_logger() {
    use log::warn;

    let log_file = File::create("debug.log").unwrap();
    match Builder::new()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .filter(None, log::LevelFilter::Trace)
        .try_init()
    {
        Ok(_) => (),
        Err(e) => warn!("env_logger was initialized bevore! Error: {}", e),
    };
}

fn main() -> io::Result<()> {
    // Check if the application is running in debug mode
    #[cfg(debug_assertions)]
    {
        init_logger();
        info!("Debug mode enabled");
    }

    // Check if the application is running in release mode
    #[cfg(not(debug_assertions))]
    {
        env_logger::init();
    }

    enable_bracketed_paste();

    let mut terminal: ratatui::Terminal<ratatui::prelude::CrosstermBackend<io::Stdout>> =
        ratatui::init();

    // Create a new instance of the application and run it.
    let app_result = Tipp10W::new().run(&mut terminal);

    ratatui::restore();

    disable_bracketed_paste();

    app_result
}
