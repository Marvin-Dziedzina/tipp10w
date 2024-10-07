mod lesson;
mod lesson_selection;

use std::fmt::Display;

use chrono::{offset::LocalResult, DateTime, Local, NaiveDateTime, TimeZone};
pub use lesson::Lesson;
pub use lesson_selection::LessonSelection;
use log::{error, info, warn};
use rusqlite::{params, Connection, OptionalExtension};

/// Get all saved lessons.
pub fn get_lessons(conn: &Connection) -> Result<Vec<Lesson>, rusqlite::Error> {
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
        Ok(Lesson::new(
            row.get(0)?,
            LessonSelection::from_lesson_name(&row.get::<_, String>(1)?),
            row.get::<_, String>(2)?.parse::<u64>().unwrap(),
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
    strokes: usize,
    errors: usize,
    timelen: usize,
    timestamp: u64,
) -> Result<usize, SQLiteError> {
    let lesson = LessonSelection::from_lesson_id(lesson_id);

    match conn.execute(
        "INSERT INTO user_lesson_list (user_lesson_lesson, user_lesson_timelen, user_lesson_tokenlen, user_lesson_strokesnum, user_lesson_errornum, user_lesson_timestamp, user_lesson_type, user_lesson_name) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![lesson.get_user_lesson(), timelen, strokes, strokes, errors, timestamp, 0, lesson.get_lesson_name()],
    ) {
        Ok(_) => (),
        Err(e) => {
            error!("Could not append lesson to database!");
            return Err(SQLiteError::RusqliteError(e));
        }
    };

    reset_ids(conn)?;

    info!("Insertion completed!");
    Ok(get_last_lesson_id(conn)?)
}

/// Update a lesson in the table.
pub fn update_lesson(
    conn: &Connection,
    id: usize,
    lesson_id: u8,
    strokes: usize,
    errors: usize,
    timelen: usize,
    timestamp: u64,
) -> Result<(), SQLiteError> {
    let lesson = LessonSelection::from_lesson_id(lesson_id);

    match conn.execute(
        "UPDATE user_lesson_list SET user_lesson_lesson = ?1, user_lesson_timelen = ?2, user_lesson_tokenlen = ?3, user_lesson_strokesnum = ?4, user_lesson_errornum = ?5, user_lesson_timestamp = ?6, user_lesson_type = ?7, user_lesson_name = ?8 WHERE user_lesson_id = ?9",
        params![lesson.get_user_lesson(), timelen, strokes, strokes, errors, timestamp, 0, lesson.get_lesson_name(), id],
    ) {
        Ok(_) => (),
        Err(e) => {
            error!("Could not update lesson in database!");
            return Err(SQLiteError::RusqliteError(e));
        }
    };

    info!("Update completed!");
    Ok(())
}

/// Delete a lesson to the table.
pub fn delete_lesson(conn: &Connection, id: usize) -> Result<(), SQLiteError> {
    match conn.execute(
        "DELETE FROM user_lesson_list WHERE user_lesson_id = ?1",
        params![id],
    ) {
        Ok(_) => (),
        Err(e) => {
            error!("Could not delete lesson!");
            return Err(SQLiteError::RusqliteError(e));
        }
    };

    reset_ids(conn)?;

    info!("Lesson deletetion completed! Lesson ID: {}", id);
    Ok(())
}

/// Get a lesson by id.
pub fn get_last_lesson_id(conn: &Connection) -> Result<usize, SQLiteError> {
    // Get max lessons.
    let mut stmt = match conn.prepare("SELECT MAX(user_lesson_id) FROM user_lesson_list") {
        Ok(stmt) => stmt,
        Err(e) => {
            error!("Could not prepair sql query for deletion!");
            return Err(SQLiteError::RusqliteError(e));
        }
    };
    let last_lesson_id_result = match stmt.query_row(params![], |row| row.get(0)).optional() {
        Ok(last_lesson_id_result) => last_lesson_id_result,
        Err(e) => {
            error!("Could not find any lessons!");
            return Err(SQLiteError::RusqliteError(e));
        }
    };

    match last_lesson_id_result {
        Some(last_lesson_id) => Ok(last_lesson_id),
        None => Err(SQLiteError::NoLessons("No lessons found!".to_string())),
    }
}

/// Errors that can occur while working with SQLite.
#[derive(Debug)]
pub enum SQLiteError {
    RusqliteError(rusqlite::Error),
    NoLessons(String),
}
impl Display for SQLiteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SQLiteError::NoLessons(e) => write!(f, "No lessons to delete from! Error: {}", e),
            SQLiteError::RusqliteError(e) => write!(f, "Rusqlite Error: {}", e),
        }
    }
}

/// Get the tipp10 timestamp.
pub fn get_timestamp() -> u64 {
    let now = Local::now();

    get_timestamp_from_now(&now)
}

/// Get tipp10 timestamp from DateTime.
pub fn get_timestamp_from_now(datetime: &DateTime<Local>) -> u64 {
    datetime
        .format("%Y%m%d%H%M%S")
        .to_string()
        .parse::<u64>()
        .expect("Call the IT support now! This error can not happen!")
}

/// Get tipp10 timestamp from string.
pub fn get_datetime_tipp10_format_from_str(datetime_string: &str) -> Result<u64, ()> {
    // Ensure that the entered DateTime is a valid date
    Ok(get_timestamp_from_now(
        match &Local.from_local_datetime(
            match &NaiveDateTime::parse_from_str(datetime_string, "%Y%m%d%H%M%S") {
                Ok(datetime) => datetime,
                Err(_) => return Err(()),
            },
        ) {
            LocalResult::Single(datetime) => datetime,
            _ => return Err(()),
        },
    ))
}

/// Reset the ids of the lessons.
pub fn reset_ids(conn: &Connection) -> Result<(), SQLiteError> {
    conn.execute(
        "CREATE TEMPORARY TABLE temp_table AS SELECT * FROM user_lesson_list",
        params![],
    )
    .map_err(|e| SQLiteError::RusqliteError(e))?;
    conn.execute("DELETE FROM user_lesson_list", params![])
        .map_err(|e| SQLiteError::RusqliteError(e))?;

    let mut stmt = conn.prepare("INSERT INTO user_lesson_list (user_lesson_id, user_lesson_lesson, user_lesson_timelen, user_lesson_tokenlen, user_lesson_strokesnum, user_lesson_errornum, user_lesson_timestamp, user_lesson_type, user_lesson_name) SELECT row_number() OVER (ORDER BY user_lesson_id) - 1, user_lesson_lesson, user_lesson_timelen, user_lesson_tokenlen, user_lesson_strokesnum, user_lesson_errornum, user_lesson_timestamp, user_lesson_type, user_lesson_name FROM temp_table").map_err(|e| SQLiteError::RusqliteError(e))?;
    stmt.execute(params![])
        .map_err(|e| SQLiteError::RusqliteError(e))?;

    conn.execute("DROP TABLE temp_table", params![])
        .map_err(|e| SQLiteError::RusqliteError(e))?;

    info!("IDs reset completed!");
    Ok(())
}
