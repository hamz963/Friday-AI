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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagDocumentChunk {
    pub id: String,
    pub file_path: String,
    pub chunk_index: usize,
    pub content: String,
    pub score: f32,
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

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS rag_chunks (
                id TEXT PRIMARY KEY,
                file_path TEXT NOT NULL,
                chunk_index INTEGER NOT NULL,
                content TEXT NOT NULL,
                timestamp TEXT NOT NULL
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

    pub fn index_document_content(&self, file_path: &str, content: &str) -> Result<usize> {
        let now = chrono::Utc::now().to_rfc3339();
        let chunks: Vec<&str> = content.split("\n\n").filter(|c| !c.trim().is_empty()).collect();
        let mut count = 0;

        for (idx, chunk_text) in chunks.iter().enumerate() {
            let chunk_id = format!("{}:{}", file_path, idx);
            self.conn.execute(
                "INSERT OR REPLACE INTO rag_chunks (id, file_path, chunk_index, content, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![chunk_id, file_path, idx as i64, chunk_text.trim(), now],
            )?;
            count += 1;
        }
        Ok(count)
    }

    pub fn rag_vector_search(&self, query: &str, top_k: usize) -> Result<Vec<RagDocumentChunk>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, file_path, chunk_index, content FROM rag_chunks"
        )?;

        let query_words: Vec<String> = query.to_lowercase().split_whitespace().map(|s| s.to_string()).collect();
        let chunk_iter = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let file_path: String = row.get(1)?;
            let chunk_index: i64 = row.get(2)?;
            let content: String = row.get(3)?;

            let content_lower = content.to_lowercase();
            let mut score: f32 = 0.0;
            for word in &query_words {
                if content_lower.contains(word) {
                    score += 1.0;
                }
            }

            Ok(RagDocumentChunk {
                id,
                file_path,
                chunk_index: chunk_index as usize,
                content,
                score,
            })
        })?;

        let mut results = Vec::new();
        for chunk in chunk_iter {
            let c = chunk?;
            if c.score > 0.0 {
                results.push(c);
            }
        }

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(top_k);
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_store_in_memory() {
        let store = MemoryStore::new_in_memory().unwrap();
        
        store.set_preference("theme", "dark").unwrap();
        let theme = store.get_preference("theme").unwrap();
        assert_eq!(theme, Some("dark".to_string()));

        let missing = store.get_preference("non-existent").unwrap();
        assert_eq!(missing, None);

        store.save_message("msg-1", "user", "hello").unwrap();
        store.save_message("msg-2", "assistant", "hi user").unwrap();
        
        let history = store.get_recent_messages(10).unwrap();
        assert_eq!(history.len(), 2);

        // Test Local Vector RAG indexing & search
        let count = store.index_document_content("PROPOSAL.md", "NOVA OS is a local Rust engine.\n\nIt features zero data leakage.").unwrap();
        assert_eq!(count, 2);

        let rag_res = store.rag_vector_search("Rust engine", 5).unwrap();
        assert!(!rag_res.is_empty());
        assert!(rag_res[0].content.contains("Rust engine"));
    }
}
