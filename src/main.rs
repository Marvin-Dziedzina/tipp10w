use std::{
    io::{stdin, stdout, Write},
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
    let conn = Connection::open("portable/tipp10v2.db").expect("Could not connect to database!");

    loop {
        let action = Action::from(&get_input(">>> ", ""));

        match action {
            Action::Show => print_lessons(&conn),
            Action::Append => append_lesson(&conn),
            Action::Delete => delete_lesson(&conn),
            Action::Exit => break,
        };
    }

    println!();
    println!("Bye (^o^)丿 ");

    thread::sleep(time::Duration::from_secs(3));
}

fn print_lessons(conn: &Connection) {
    let mut stmt = conn
        .prepare(
            "SELECT user_lesson_id, user_lesson_name, user_lesson_timestamp, user_lesson_timelen, user_lesson_strokesnum, user_lesson_errornum FROM user_lesson_list",
        )
        .expect("Could not get all lessons from database!");
    let lessons_iter = stmt
        .query_map(params![], |row| {
            Ok(LessonSaveData::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2).unwrap(),
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
            ))
        })
        .expect("Could not get all lessons!");

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
    let lesson_id = get_input("Lesson ID", "18")
        .parse::<u8>()
        .expect("Lesson ID needs to be a numerical value!");
    let lesson = Lesson::from_lesson_id(lesson_id);

    let lesson_strokes = get_input("Strokes: ", "")
        .parse::<u32>()
        .expect("Strokes needs to be a numerical value!");

    let lesson_errornum = get_input("Errors: ", "")
        .parse::<u32>()
        .expect("The error count needs to be a numerical value!");

    let lesson_timelen = get_input("Lesson lenght in seconds", "600")
        .parse::<u32>()
        .expect("Lesson lenght needs to be a numerical value!");

    conn.execute(
        "INSERT INTO user_lesson_list (user_lesson_lesson, user_lesson_timelen, user_lesson_tokenlen, user_lesson_strokesnum, user_lesson_errornum, user_lesson_timestamp, user_lesson_type, user_lesson_name) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![lesson.get_user_lesson(), lesson_timelen, lesson_strokes, lesson_strokes, lesson_errornum, get_timestamp(), 0, lesson.get_lesson_name()],
    ).expect("Could not write new data to database!");

    println!("Insertion completed!")
}

fn delete_lesson(conn: &Connection) {
    // Get max lessons.
    let mut stmt = conn
        .prepare("SELECT MAX(user_lesson_id) FROM user_lesson_list")
        .expect("Could not prepair sql query for deletion!");
    let last_lesson_id: u32 = match stmt
        .query_row(params![], |row| row.get(0))
        .optional()
        .expect("Could not get entry count!")
    {
        Some(last_lesson_id) => last_lesson_id,
        None => return,
    };

    conn.execute(
        "DELETE FROM user_lesson_list WHERE user_lesson_id = ?1",
        params![last_lesson_id],
    )
    .expect("Could not delete row from database!");

    println!(
        "Last lesson deletetion completed! Lesson ID: {}",
        last_lesson_id
    );
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
