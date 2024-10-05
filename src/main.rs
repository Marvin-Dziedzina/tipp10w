use std::io::{self, Write};

pub mod events;
pub mod state;
mod tipp10;
mod tipp10w;
pub mod ui;
pub mod widgets;

use tipp10w::Tipp10W;

fn enable_bracketed_paste() {
    print!("\x1b[?2004h");
    io::stdout().flush().unwrap();
}

fn disable_bracketed_paste() {
    print!("\x1b[?2004l");
    io::stdout().flush().unwrap();
}

fn main() -> io::Result<()> {
    env_logger::init();

    enable_bracketed_paste();

    let mut terminal = ratatui::init();
    let app_result = Tipp10W::new().run(&mut terminal);

    ratatui::restore();
    disable_bracketed_paste();

    app_result
}

// fn print_help() {
//     println!();
//     println!("--- !(^^)! Help ＼(◎o◎)／！ ---");
//     println!("Actions:");
//     println!("  When `>>>` appears you are asked what action you want to do.");
//     println!("  There are 5 possible actions.");
//     println!("  `h` for help.");
//     println!("  `p` for print done lessons.");
//     println!("  `a` for appending a new lesson.");
//     println!("  `d` for the deletion of a lesson.");
//     println!("  `e` for exiting the application.");
//     println!("Appending:");
//     println!("  Lesson ID is the number that is in the lessons name.");
//     println!("  Strokes are the total number of key strokes that occured in the lesson.");
//     println!("  Errors are the total error count that occured in the lesson.");
//     println!("  Lesson lenght in seconds is the lenght of the lesson in seconds.");
//     println!("Deletion:");
//     println!("  ID is the id of the lesson you want to delete.");
//     println!();
//     println!("      ᕕ( ᐛ )ᕗ");
//     println!();
//     println!();
// }
