use rusqlite::{named_params, Connection, Error, Result};
use std::fs;
use std::path::PathBuf;

pub mod models {
    #[derive(Debug, Clone)]
    pub struct Todo {
        pub id: String,
        pub title: String,
        pub completed: bool,
    }
}

use models::Todo;

pub fn init() -> Result<Connection, Error> {
    let storage_dir = get_or_create_app_dir("todoctl/storage/")
        .expect("Failed to create app dir")
        .to_string_lossy()
        .to_string();

    let db_path = format!("{storage_dir}/.todos.db");
    let conn = Connection::open(db_path)?;
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id        TEXT PRIMARY KEY,
            title     TEXT NOT NULL,
            completed BOOLEAN NOT NULL
        )
        "#,
        (),
    )?;
    Ok(conn)
}

fn get_or_create_app_dir(
    dir_name: &str,
) -> std::result::Result<PathBuf, Box<dyn std::error::Error>> {
    let app_dir = dirs::data_local_dir()
        .map(|pb| pb.join(dir_name))
        .ok_or("Could not determine local data directory")?;

    // Create directory if it doesn't exist
    fs::create_dir_all(&app_dir).expect("Failed to create data directory");

    Ok(app_dir)
}

pub fn add_todo(conn: &Connection, todo: Todo) -> Result<usize, Error> {
    conn.execute(
        "INSERT INTO todos (id, title, completed) VALUES (?1, ?2, ?3)",
        [&todo.id, &todo.title, &(todo.completed as i32).to_string()],
    )
}

pub fn complete_todo(conn: &Connection, id: &str) -> Result<usize, Error> {
    conn.execute("UPDATE todos SET completed = 1 WHERE id = ?1", [&id])
}

pub fn delete_todo(conn: &Connection, id: &str) -> Result<usize, Error> {
    conn.execute("DELETE FROM todos WHERE id = ?1", [&id])
}

pub fn show_all(conn: &Connection) -> Result<Vec<Todo>, Error> {
    let mut stmt = conn.prepare("SELECT id, title, completed FROM todos")?;
    let rows = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get::<_, i32>(2)? != 0,
        })
    })?;
    rows.collect()
}

pub fn show_one(conn: &Connection, id: &str) -> Result<Vec<Todo>, Error> {
    let mut stmt = conn.prepare("SELECT id, title, completed FROM todos WHERE id = ?1")?;
    let rows = stmt.query_map([id], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get::<_, i32>(2)? != 0,
        })
    })?;
    rows.collect()
}

pub fn update_todo(
    conn: &Connection,
    id: &str,
    new_title: Option<&str>,
    new_completed: Option<bool>,
) -> Result<usize, Error> {
    // Build dynamic SQL with named parameters
    let mut parts = Vec::new();

    if new_title.is_some() {
        parts.push("title = :title");
    }

    if new_completed.is_some() {
        parts.push("completed = :completed");
    }

    if parts.is_empty() {
        return Ok(0);
    }

    let sql = format!("UPDATE todos SET {} WHERE id = :id", parts.join(", "));

    // Create the statement
    let mut stmt = conn.prepare(&sql)?;

    // Convert bool to i32 for SQLite compatibility if needed
    let completed_int = new_completed.map(|c| if c { 1 } else { 0 });

    // Execute with different named parameters depending on what was provided
    match (new_title, completed_int) {
        (Some(title), Some(completed)) => stmt.execute(named_params! {
            ":title": title,
            ":completed": completed,
            ":id": id,
        }),
        (Some(title), None) => stmt.execute(named_params! {
            ":title": title,
            ":id": id,
        }),
        (None, Some(completed)) => stmt.execute(named_params! {
            ":completed": completed,
            ":id": id,
        }),
        _ => Ok(0), // Should never reach here due to the empty check above
    }
}
