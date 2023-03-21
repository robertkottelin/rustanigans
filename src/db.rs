use rusqlite::{Connection, Result};
use std::path::Path;

pub async fn connect() -> Result<Connection> {
    let db_path = Path::new("chat.db");
    let connection = Connection::open(db_path)?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )",
        [],
    )?;

    Ok(connection)
}
