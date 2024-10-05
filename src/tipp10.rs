mod lesson;
mod lesson_savedata;

use std::fmt::Display;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
pub use lesson::Lesson;
pub use lesson_savedata::LessonSaveData;
use log::{error, info, warn};
use rusqlite::{params, Connection, OptionalExtension};

/// Get all saved lessons.
pub fn get_lessons(conn: &Connection) -> Result<Vec<LessonSaveData>, rusqlite::Error> {
    let mut stmt = match conn
            .prepare(
                "SELECT user_lesson_id, user_lesson_name, user_lesson_timestamp, user_lesson_timelen, user_lesson_strokesnum, user_lesson_errornum FROM user_lesson_list",
            ) {
                Ok(stmt) => stmt,
                Err(e) => {
                    error!("Could not get all lessons from database! Error: {}", e);
                    return Err(e);
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
        Err(e) => {
            error!("Could not get all lessons! Error: {}", e);
            return Err(e);
        }
    };

    let mut lessons = Vec::new();
    for lesson in lessons_iter {
        match lesson {
            Ok(lesson) => lessons.push(lesson),
            Err(e) => {
                warn!("Could not get all lessons! Error: {}", e);
                continue;
            }
        }
    }

    Ok(lessons)
}

/// Append a lesson to the table.
pub fn append_lesson(
    conn: &Connection,
    lesson_id: u8,
    strokes: u32,
    errors: u32,
    timelen: u32,
    timestamp: u64,
) -> Result<(), rusqlite::Error> {
    let lesson = Lesson::from_lesson_id(lesson_id);

    match conn.execute(
        "INSERT INTO user_lesson_list (user_lesson_lesson, user_lesson_timelen, user_lesson_tokenlen, user_lesson_strokesnum, user_lesson_errornum, user_lesson_timestamp, user_lesson_type, user_lesson_name) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![lesson.get_user_lesson(), timelen, strokes, strokes, errors, timestamp, 0, lesson.get_lesson_name()],
    ) {
        Ok(_) => (),
        Err(e) => {
            error!("Could not append lesson to database!");
            return Err(e);
        }
    };

    info!("Insertion completed!");
    Ok(())
}

/// Delete a lesson to the table.
pub fn delete_lesson(conn: &Connection, id: u32) -> Result<(), DeleteError> {
    match conn.execute(
        "DELETE FROM user_lesson_list WHERE user_lesson_id = ?1",
        params![id],
    ) {
        Ok(_) => (),
        Err(e) => {
            error!("Could not delete lesson!");
            return Err(DeleteError::RusqliteError(e));
        }
    };

    info!("Lesson deletetion completed! Lesson ID: {}", id);
    Ok(())
}

pub fn get_last_lesson_id(conn: &Connection) -> Result<Option<u32>, DeleteError> {
    // Get max lessons.
    let mut stmt = match conn.prepare("SELECT MAX(user_lesson_id) FROM user_lesson_list") {
        Ok(stmt) => stmt,
        Err(e) => {
            error!("Could not prepair sql query for deletion!");
            return Err(DeleteError::RusqliteError(e));
        }
    };
    let last_lesson_id_result = match stmt.query_row(params![], |row| row.get(0)).optional() {
        Ok(last_lesson_id_result) => last_lesson_id_result,
        Err(e) => {
            error!("Could not find any lessons!");
            return Err(DeleteError::RusqliteError(e));
        }
    };

    match last_lesson_id_result {
        Some(last_lesson_id) => Ok(Some(last_lesson_id)),
        None => {
            info!("No lesson entries!");
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub enum DeleteError {
    RusqliteError(rusqlite::Error),
    NoLessons(String),
}
impl Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteError::NoLessons(e) => write!(f, "No lessons to delete from! Error: {}", e),
            DeleteError::RusqliteError(e) => write!(f, "Rusqlite Error: {}", e),
        }
    }
}

/// Get the tipp10 timestamp.
pub fn get_timestamp() -> u64 {
    let now = Local::now();

    get_timestamp_from_now(&now)
}

pub fn get_timestamp_from_now(datetime: &DateTime<Local>) -> u64 {
    datetime
        .format("%Y%m%d%H%M%S")
        .to_string()
        .parse::<u64>()
        .expect("Call the IT support now! This error can not happen!")
}

/// Get tipp10 timestamp from string.
pub fn get_datetime_tipp10_format(datetime_string: &str) -> u64 {
    // Ensure that the entered DateTime is a valid date
    get_timestamp_from_now(
        &Local
            .from_local_datetime(
                &NaiveDateTime::parse_from_str(datetime_string, "%Y%m%d%H%M%S")
                    .expect("Wrong time format!"),
            )
            .unwrap(),
    )
}
