use std::{
    io::{stdin, stdout, Write},
    path::PathBuf,
    str::FromStr,
    thread, time,
};

use chrono::{self, DateTime, Local, NaiveDateTime, TimeZone};
use rusqlite::{params, Connection, OptionalExtension};

mod action;
mod lesson;
mod lesson_savedata;

use action::Action;
use lesson::Lesson;
use lesson_savedata::LessonSaveData;

fn main() {
    let db_path_string = get_input("Path to Tipp10: ", "");
    let mut db_path =
        PathBuf::from_str(&db_path_string).expect("Tipp10 Path needs to be a valid path!");
    db_path.push("portable");
    db_path.push("tipp10v2.db");

    let conn = Connection::open(db_path).expect("Could not connect to database!");

    println!("Input `h` for help or `e` for exit.");

    loop {
        println!("Actions: `h`, `p`, `a`, `d`, `e`");
        let action = Action::from(&get_input(">>> ", ""));

        match action {
            Action::Help => print_help(),
            Action::Print => print_lessons(&conn),
            Action::Append => append_lesson(&conn),
            Action::Delete => delete_lesson(&conn),
            Action::Exit => break,
            Action::Invalid => {
                println!("Invalid action selected!");
                continue;
            }
        };
    }

    println!();
    println!("Bye (^o^)丿 ");

    thread::sleep(time::Duration::from_secs(3));
}

fn print_help() {
    println!();
    println!("--- !(^^)! Help ＼(◎o◎)／！ ---");
    println!("Actions:");
    println!("  When `>>>` appears you are asked what action you want to do.");
    println!("  There are 5 possible actions.");
    println!("  `h` for help.");
    println!("  `p` for print done lessons.");
    println!("  `a` for appending a new lesson.");
    println!("  `d` for the deletion of a lesson.");
    println!("  `e` for exiting the application.");
    println!("Appending:");
    println!("  Lesson ID is the number that is in the lessons name.");
    println!("  Strokes are the total number of key strokes that occured in the lesson.");
    println!("  Errors are the total error count that occured in the lesson.");
    println!("  Lesson lenght in seconds is the lenght of the lesson in seconds.");
    println!("Deletion:");
    println!("  ID is the id of the lesson you want to delete.");
    println!();
    println!("      ᕕ( ᐛ )ᕗ");
    println!();
    println!();
}

fn print_lessons(conn: &Connection) {
    let mut stmt = match conn
        .prepare(
            "SELECT user_lesson_id, user_lesson_name, user_lesson_timestamp, user_lesson_timelen, user_lesson_strokesnum, user_lesson_errornum FROM user_lesson_list",
        ) {
            Ok(stmt) => stmt,
            Err(_) => {
                println!("Could not get all lessons from database!");
                return;
            }
        };
    let lessons_iter = match stmt.query_map(params![], |row| {
        Ok(LessonSaveData::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
        ))
    }) {
        Ok(stmt) => stmt,
        Err(_) => {
            println!("Could not get all lessons!");
            return;
        }
    };

    println!();
    println!();
    println!("--- (._.) Lessons (._.) ---");

    for lesson in lessons_iter {
        let lesson = match lesson {
            Ok(lesson) => lesson,
            Err(_) => {
                println!("Could not read this lessons savedata! It will be skipped.");
                continue;
            }
        };

        println!("{}", lesson);
    }

    println!();
    println!("--- :3ミ ---");
    println!();
    println!();
}

fn append_lesson(conn: &Connection) {
    let lesson_id = match get_input("Lesson ID", "18").parse::<u8>() {
        Ok(lesson_id) => lesson_id,
        Err(_) => {
            println!("Lesson ID needs to be a numerical value!");
            return;
        }
    };
    let lesson = Lesson::from_lesson_id(lesson_id);

    let lesson_strokes = match get_input("Strokes: ", "").parse::<u32>() {
        Ok(lesson_strokes) => lesson_strokes,
        Err(_) => {
            println!("Strokes needs to be a numerical value!");
            return;
        }
    };

    let lesson_errornum = match get_input("Errors: ", "").parse::<u32>() {
        Ok(lesson_errornum) => lesson_errornum,
        Err(_) => {
            println!("The error count needs to be a numerical value!");
            return;
        }
    };

    let lesson_timelen = match get_input("Lesson lenght in seconds", "600").parse::<u32>() {
        Ok(lesson_timelen) => lesson_timelen,
        Err(_) => {
            println!("Lesson lenght needs to be a numerical value!");
            return;
        }
    };

    match conn.execute(
        "INSERT INTO user_lesson_list (user_lesson_lesson, user_lesson_timelen, user_lesson_tokenlen, user_lesson_strokesnum, user_lesson_errornum, user_lesson_timestamp, user_lesson_type, user_lesson_name) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![lesson.get_user_lesson(), lesson_timelen, lesson_strokes, lesson_strokes, lesson_errornum, get_timestamp(), 0, lesson.get_lesson_name()],
    ) {
        Ok(_) => (),
        Err(_) => {
            println!("Could not append lesson to database!");
            return;
        }
    };

    println!("Insertion completed!")
}

fn delete_lesson(conn: &Connection) {
    print_lessons(conn);

    // Get max lessons.
    let mut stmt = match conn.prepare("SELECT MAX(user_lesson_id) FROM user_lesson_list") {
        Ok(stmt) => stmt,
        Err(_) => {
            println!("Could not prepair sql query for deletion!");
            return;
        }
    };
    let last_lesson_id_result = match stmt.query_row(params![], |row| row.get(0)).optional() {
        Ok(last_lesson_id_result) => last_lesson_id_result,
        Err(_) => {
            println!("Could not find any lessons!");
            return;
        }
    };

    let last_lesson_id: u32 = match last_lesson_id_result {
        Some(last_lesson_id) => last_lesson_id,
        None => return,
    };

    let id = get_input("ID", &last_lesson_id.to_string());

    match conn.execute(
        "DELETE FROM user_lesson_list WHERE user_lesson_id = ?1",
        params![id],
    ) {
        Ok(_) => (),
        Err(_) => {
            println!("Could not delete lesson!");
            return;
        }
    };

    println!("Last lesson deletetion completed! Lesson ID: {}", id);
}

/// Get user input.
fn get_input(question: &str, default: &str) -> String {
    // Print input question.
    if !default.is_empty() {
        print!("{} (Default: {}): ", question, default);
    } else {
        print!("{}", question);
    };
    stdout().flush().expect("Could not flush stdout!");

    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("Could not get user input!");

    buf = buf.trim().to_string();

    // Use the default value if nothing was entered.
    if !buf.is_empty() || default.is_empty() {
        buf
    } else {
        default.to_string()
    }
}

/// Get the tipp10 timestamp.
fn get_timestamp() -> u64 {
    let now = chrono::Local::now();

    let current_time = get_datetime_tipp10_format(now);

    let datetime_string = get_input("Time (Format %Y%m%d%H%M%S)", &current_time.to_string());

    // Ensure that the entered DateTime is a valid date
    get_datetime_tipp10_format(
        Local
            .from_local_datetime(
                &NaiveDateTime::parse_from_str(&datetime_string, "%Y%m%d%H%M%S")
                    .expect("Wrong time format!"),
            )
            .unwrap(),
    )
}

fn get_datetime_tipp10_format(datetime: DateTime<Local>) -> u64 {
    datetime
        .format("%Y%m%d%H%M%S")
        .to_string()
        .parse::<u64>()
        .expect("Call the IT support now! This error can not happen!")
}
