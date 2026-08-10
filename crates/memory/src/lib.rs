use rusqlite::{params, Connection, Result};
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    pub id: String,
    pub role: String,
    pub content: String,
    pub timestamp: String,
}

pub struct MemoryStore {
    conn: Connection,
}

impl MemoryStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let store = Self { conn };
        store.initialize_tables()?;
        Ok(store)
    }

    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let store = Self { conn };
        store.initialize_tables()?;
        Ok(store)
    }

    fn initialize_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS conversation (
                id TEXT PRIMARY KEY,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS preference (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    pub fn save_message(&self, id: &str, role: &str, content: &str) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT OR REPLACE INTO conversation (id, role, content, timestamp) VALUES (?1, ?2, ?3, ?4)",
            params![id, role, content, now],
        )?;
        Ok(())
    }

    pub fn get_recent_messages(&self, limit: usize) -> Result<Vec<StoredMessage>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, role, content, timestamp FROM conversation ORDER BY timestamp DESC LIMIT ?1"
        )?;
        let message_iter = stmt.query_map(params![limit as i64], |row| {
            Ok(StoredMessage {
                id: row.get(0)?,
                role: row.get(1)?,
                content: row.get(2)?,
                timestamp: row.get(3)?,
            })
        })?;

        let mut messages = Vec::new();
        for msg in message_iter {
            messages.push(msg?);
        }
        messages.reverse();
        Ok(messages)
    }

    pub fn get_preference(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT value FROM preference WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        if let Some(row) = rows.next()? {
            let val: String = row.get(0)?;
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }

    pub fn set_preference(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO preference (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_store_in_memory() {
        let store = MemoryStore::new_in_memory().unwrap();
        
        // Test preferences
        store.set_preference("theme", "dark").unwrap();
        let theme = store.get_preference("theme").unwrap();
        assert_eq!(theme, Some("dark".to_string()));

        // Test non-existent preference
        let missing = store.get_preference("non-existent").unwrap();
        assert_eq!(missing, None);

        // Test conversation history
        store.save_message("msg-1", "user", "hello").unwrap();
        store.save_message("msg-2", "assistant", "hi user").unwrap();
        
        let history = store.get_recent_messages(10).unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].content, "hello");
        assert_eq!(history[1].content, "hi user");
    }
}
