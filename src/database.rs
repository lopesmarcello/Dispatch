use directories::ProjectDirs;
use rusqlite::{Connection, Result, params};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct HistoryItem {
    pub id: i64,
    pub method: String,
    pub url: String,
    pub request_body: String,
    pub request_headers: String,
    pub response_body: String,
    pub response_headers: String,
    pub status: String,
    pub time: String,
    pub size: String,
    pub timestamp: String,
}

#[derive(Debug)]
pub struct Collection {
    pub id: i64,
    pub name: String
}

#[derive(Debug)]
pub struct CollectionItem {
    pub id: i64,
    pub collection_id: i64,
    pub name: String,
    pub method: String,
    pub url: String,
    pub body: String,
    pub headers: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = if let Some(proj_dirs) = ProjectDirs::from("com", "example", "dispatch") {
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
                request_body TEXT,
                request_headers TEXT,
                response_body TEXT,
                response_headers TEXT,
                status TEXT,
                time TEXT,
                size TEXT,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

       conn.execute(
            "CREATE TABLE IF NOT EXISTS collection_items (
                id INTEGER PRIMARY KEY,
                collection_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                method TEXT NOT NULL,
                url TEXT NOT NULL,
                body TEXT,
                headers TEXT,
                FOREIGN KEY(collection_id) REFERENCES collections(id) ON DELETE CASCADE
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS collections (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            )",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn save_exchange(
        &self,
        method: &str,
        url: &str,
        req_body: &str,
        req_headers: &str,
        res_body: &str,
        res_headers: &str,
        status: &str,
        time: &str,
        size: &str,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO history (
                method, url, request_body, request_headers, 
                response_body, response_headers, status, time, size
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                method,
                url,
                req_body,
                req_headers,
                res_body,
                res_headers,
                status,
                time,
                size
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_history(&self) -> Result<Vec<HistoryItem>> {
        let mut stmt = self.conn.prepare("SELECT id, method, url, request_body, request_headers, response_body, response_headers, status, time, size, timestamp FROM history ORDER BY id DESC LIMIT 50")?;

        let rows = stmt.query_map([], |row| {
            Ok(HistoryItem {
                id: row.get(0)?,
                method: row.get(1)?,
                url: row.get(2)?,
                request_body: row.get(3).unwrap_or_default(),
                request_headers: row.get(4).unwrap_or_default(),
                response_body: row.get(5).unwrap_or_default(),
                response_headers: row.get(6).unwrap_or_default(),
                status: row.get(7).unwrap_or_default(),
                time: row.get(8).unwrap_or_default(),
                size: row.get(9).unwrap_or_default(),
                timestamp: row.get(10)?,
            })
        })?;

        let mut items = Vec::new();
        for row in rows {
            items.push(row?);
        }
        Ok(items)
    }

    pub fn get_request_by_id(&self, id: i64) -> Result<HistoryItem> {
        let mut stmt = self.conn.prepare("SELECT id, method, url, request_body, request_headers, response_body, response_headers, status, time, size, timestamp FROM history WHERE id = ?1")?;

        let mut rows = stmt.query_map(params![id], |row| {
            Ok(HistoryItem {
                id: row.get(0)?,
                method: row.get(1)?,
                url: row.get(2)?,
                request_body: row.get(3).unwrap_or_default(),
                request_headers: row.get(4).unwrap_or_default(),
                response_body: row.get(5).unwrap_or_default(),
                response_headers: row.get(6).unwrap_or_default(),
                status: row.get(7).unwrap_or_default(),
                time: row.get(8).unwrap_or_default(),
                size: row.get(9).unwrap_or_default(),
                timestamp: row.get(10)?,
            })
        })?;

        if let Some(row) = rows.next() {
            row
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    pub fn clear_history(&self) -> Result<()> {
        self.conn.execute("DELETE FROM history", [])?;
        Ok(())
    }

    pub fn create_collection(&self, name: &str) -> Result<i64> {
        self.conn.execute("INSERT INTO collections (name) VALUES (?1)", params![name])?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_collections(&self) -> Result<Vec<Collection>> {
        let mut stmt = self.conn.prepare("SELECT id, name FROM collections ORDER BY name ASC")?;
        let rows = stmt.query_map([], |row| {
            Ok(Collection { id: row.get(0)?, name: row.get(1)? })
        })?;
        let mut items = Vec::new();
        for row in rows { items.push(row?); }
        Ok(items)
    }

    pub fn save_to_collection(&self, col_id: i64, name: &str, method: &str, url: &str, body: &str, headers: &str) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO collection_items (collection_id, name, method, url, body, headers) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![col_id, name, method, url, body, headers],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_collection_items(&self, col_id: i64) -> Result<Vec<CollectionItem>> {
        let mut stmt = self.conn.prepare("SELECT id, collection_id, name, method, url, body, headers FROM collection_items WHERE collection_id = ?1")?;
        let rows = stmt.query_map(params![col_id], |row| {
            Ok(CollectionItem {
                id: row.get(0)?,
                collection_id: row.get(1)?,
                name: row.get(2)?,
                method: row.get(3)?,
                url: row.get(4)?,
                body: row.get(5).unwrap_or_default(),
                headers: row.get(6).unwrap_or_default(),
            })
        })?;
        let mut items = Vec::new();
        for row in rows { items.push(row?); }
        Ok(items)
    }
}
