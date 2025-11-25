use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

/// Initialize the database and create the todos table if it doesn't exist
pub fn init_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            text        TEXT NOT NULL,
            completed   INTEGER NOT NULL DEFAULT 0,
            created_at  TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

/// Insert a new todo into the database
pub fn insert_todo(conn: &Connection, text: String) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO todos (text, completed, created_at) VALUES (?1, 0, ?2)",
        params![text, now],
    )?;
    Ok(())
}

/// List todos with optional filtering
pub fn list_todos(conn: &Connection, filter: Option<String>) -> Result<Vec<Todo>> {
    let query = match filter.as_deref() {
        Some("active") => {
            "SELECT id, text, completed, created_at FROM todos WHERE completed = 0 ORDER BY id"
        }
        Some("completed") => {
            "SELECT id, text, completed, created_at FROM todos WHERE completed = 1 ORDER BY id"
        }
        _ => "SELECT id, text, completed, created_at FROM todos ORDER BY id",
    };

    let mut stmt = conn.prepare(query)?;
    let todos = stmt.query_map([], |row| {
        let created_at_str: String = row.get(3)?;
        Ok(Todo {
            id: row.get(0)?,
            text: row.get(1)?,
            completed: row.get::<_, i64>(2)? != 0,
            created_at: DateTime::parse_from_rfc3339(&created_at_str)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
        })
    })?;

    todos.collect()
}

/// Toggle the completion status of a todo
pub fn toggle_todo(conn: &Connection, id: i64) -> Result<()> {
    conn.execute(
        "UPDATE todos SET completed = NOT completed WHERE id = ?1",
        params![id],
    )?;
    Ok(())
}

/// Delete a specific todo by ID
pub fn delete_todo(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM todos WHERE id = ?1", params![id])?;
    Ok(())
}

/// Delete all completed todos
pub fn clear_completed(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM todos WHERE completed = 1", [])?;
    Ok(())
}

/// Get statistics about todos (total, active, completed)
pub fn get_stats(conn: &Connection) -> Result<(usize, usize, usize)> {
    let total: i64 = conn.query_row("SELECT COUNT(*) FROM todos", [], |r| r.get(0))?;
    let completed: i64 =
        conn.query_row("SELECT COUNT(*) FROM todos WHERE completed = 1", [], |r| {
            r.get(0)
        })?;
    let active = total - completed;

    Ok((total as usize, active as usize, completed as usize))
}
