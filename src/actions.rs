use prettytable::{row, Table};
use rusqlite::Connection;
use uuid::Uuid;

use crate::db::{
    add_todo, complete_todo, delete_todo, models::Todo, show_all, show_one, update_todo,
};
use crate::utils::user_input;

/// Add a new todo, either from `--title` or interactive prompt
pub async fn add_action(conn: &Connection, title_arg: Option<String>) {
    let title = title_arg.unwrap_or_else(|| {
        println!("Add:");
        user_input("todo".to_string()).trim().to_owned()
    });

    let todo = Todo {
        id: Uuid::new_v4().to_string(),
        title: title.clone(),
        completed: false,
    };

    match add_todo(conn, todo.clone()) {
        Ok(_) => {
            let mut table = Table::new();
            table.add_row(row![b->"ID", b->"Title", b->"Completed"]);
            table.add_row(row![
                todo.id,
                todo.title,
                if todo.completed { "✅" } else { "❌" }
            ]);
            table.printstd();
        }
        Err(e) => eprintln!("Error adding todo: {}", e),
    }
}

/// List all todos
pub async fn show_action(conn: &Connection) {
    match show_all(conn) {
        Ok(todos) if todos.is_empty() => {
            println!("No todos found!");
        }
        Ok(todos) => {
            let mut table = Table::new();
            table.add_row(row![b->"ID", b->"Title", b->"Completed"]);
            for t in todos {
                table.add_row(row![
                    t.id,
                    t.title,
                    if t.completed { "✅" } else { "❌" }
                ]);
            }
            table.printstd();
        }
        Err(e) => eprintln!("Error fetching todos: {}", e),
    }
}

/// Complete a todo by ID (flag or interactive)
pub async fn complete_action(conn: &Connection, id_arg: Option<String>) {
    let id = id_arg.unwrap_or_else(|| {
        println!("Complete:");
        user_input("ID".to_string()).trim().to_owned()
    });

    if let Err(e) = complete_todo(conn, &id) {
        eprintln!("Error completing todo: {}", e);
        return;
    }

    match show_one(conn, &id) {
        Ok(mut v) if !v.is_empty() => {
            let t = v.remove(0);
            let mut table = Table::new();
            table.add_row(row![b->"ID", b->"Title", b->"Completed"]);
            table.add_row(row![
                t.id,
                t.title,
                if t.completed { "✅" } else { "❌" }
            ]);
            table.printstd();
        }
        Ok(_) => println!("No todo found with ID: {}", id),
        Err(e) => eprintln!("Error fetching todo: {}", e),
    }
}

/// Delete a todo by ID (flag or interactive)
pub async fn delete_action(conn: &Connection, id_arg: Option<String>) {
    let id = id_arg.unwrap_or_else(|| {
        println!("Delete:");
        user_input("ID".to_string()).trim().to_owned()
    });

    // Show it first
    if let Ok(mut v) = show_one(conn, &id) {
        if v.is_empty() {
            println!("No todo found with ID: {}", id);
            return;
        }
        let t = v.remove(0);
        let mut table = Table::new();
        table.add_row(row![b->"ID", b->"Title", b->"Completed"]);
        table.add_row(row![
            t.id,
            t.title,
            if t.completed { "✅" } else { "❌" }
        ]);
        table.printstd();
    }

    if let Err(e) = delete_todo(conn, &id) {
        eprintln!("Error deleting todo: {}", e);
    }
}

/// Update a todo by ID (flag or interactive)
pub async fn update_action(
    conn: &Connection,
    id_arg: Option<String>,
    title_arg: Option<String>,
    comp_arg: Option<bool>,
) {
    // 1) Determine ID
    let id = id_arg.unwrap_or_else(|| {
        println!("Update:");
        user_input("ID of todo to update".into()).trim().to_owned()
    });

    // 2) Fetch and show existing
    match show_one(conn, &id) {
        Ok(mut v) if !v.is_empty() => {
            let todo = v.remove(0);
            let mut tbl = Table::new();
            tbl.add_row(row![b->"ID", b->"Title", b->"Completed"]);
            tbl.add_row(row![
                &todo.id,
                &todo.title,
                if todo.completed { "✅" } else { "❌" }
            ]);
            tbl.printstd();
        }
        _ => {
            eprintln!("❌ No todo found with ID: {}", id);
            return;
        }
    }

    // 3) Determine new title
    let new_title = title_arg.or_else(|| {
        let input = user_input("New title (leave blank to skip)".into());
        let trimmed = input.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_owned())
        }
    });

    // 4) Determine new completed status
    let new_completed = comp_arg.or_else(|| {
        let input = user_input("Completed? (y/n, leave blank to skip)".into());
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => Some(true),
            "n" | "no" => Some(false),
            _ => None,
        }
    });

    // 5) Apply update
    match update_todo(conn, &id, new_title.as_deref(), new_completed) {
        Ok(0) => println!("No fields changed."),
        Ok(_) => println!("✅ Todo updated."),
        Err(e) => eprintln!("Error updating todo: {}", e),
    }
}
