use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use rusqlite::{Connection, Result, params};

#[derive(Debug)]
pub struct HistoryItem {
    pub id: i64,
    pub method: String,
    pub url: String,
    pub body: String,
    pub headers: String,
    pub timestamp: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = if let Some(proj_dirs) =
            ProjectDirs::from("com", "github.com/lopesmarcello", "dispatch")
        {
            let config_dir = proj_dirs.config_dir();

            if !config_dir.exists() {
                fs::create_dir_all(config_dir).unwrap_or_default();
            }

            config_dir.join("history.db")
        } else {
            PathBuf::from("history.db")
        };

        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
            id INTEGER PRIMARY KEY,
            method TEXT NOT NULL,
            url TEXT NOT NULL,
            body TEXT,
            headers TEXT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            ",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn save_request(&self, method: &str, url: &str, body: &str, headers: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO history (method, url, body, headers) VALUES (?1, ?2, ?3, ?4)",
            params![method, url, body, headers],
        )?;
        Ok(())
    }

    pub fn get_history(&self) -> Result<Vec<HistoryItem>> {
        let mut stmt = self.conn.prepare("SELECT id, method, url, body, headers, timestamp FROM history ORDER BY id DESC LIMIT 50")?;

        let rows = stmt.query_map([], |row| {
            Ok(HistoryItem {
                id: row.get(0)?,
                method: row.get(1)?,
                url: row.get(2)?,
                body: row.get(3).unwrap_or_default(),
                headers: row.get(4).unwrap_or_default(),
                timestamp: row.get(5)?,
            })
        })?;

        let mut items = Vec::new();
        for row in rows {
            items.push(row?);
        }
        Ok(items)
    }

    pub fn clear_history(&self) -> Result<()> {
        self.conn.execute("DELETE FROM history", [])?;
        Ok(())
    }
}
