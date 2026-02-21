//! Database Schema
//! 
//! Defines the SQLite schema including FTS5 virtual tables for search.

use rusqlite::{Connection, Result};
use std::path::Path;

/// Initialize the database with the required schema
pub fn init_database(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    
    // Enable WAL mode for better concurrency
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;
    
    // Create the main clipboard entries table
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS clipboard_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL,
            content_type TEXT CHECK(content_type IN ('text', 'code', 'markdown')) NOT NULL DEFAULT 'text',
            created_at INTEGER NOT NULL,
            is_pinned INTEGER NOT NULL DEFAULT 0,
            is_scrubbed INTEGER NOT NULL DEFAULT 0,
            source_app TEXT,
            hash TEXT UNIQUE
        )
        "#,
        [],
    )?;
    
    // Create index on created_at for faster recent queries
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_created_at ON clipboard_entries(created_at DESC)",
        [],
    )?;
    
    // Create index on is_pinned for faster pinned queries
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_is_pinned ON clipboard_entries(is_pinned)",
        [],
    )?;
    
    // Create FTS5 virtual table for full-text search
    // Using unicode61 tokenizer with remove_diacritics for Arabic support
    conn.execute_batch(
        r#"
        CREATE VIRTUAL TABLE IF NOT EXISTS clipboard_fts USING fts5(
            content,
            content='clipboard_entries',
            content_rowid='id',
            tokenize='unicode61 remove_diacritics 2'
        );
        
        -- Triggers to keep FTS in sync with main table
        CREATE TRIGGER IF NOT EXISTS clipboard_ai AFTER INSERT ON clipboard_entries BEGIN
            INSERT INTO clipboard_fts(rowid, content) VALUES (new.id, new.content);
        END;
        
        CREATE TRIGGER IF NOT EXISTS clipboard_ad AFTER DELETE ON clipboard_entries BEGIN
            INSERT INTO clipboard_fts(clipboard_fts, rowid, content) VALUES('delete', old.id, old.content);
        END;
        
        CREATE TRIGGER IF NOT EXISTS clipboard_au AFTER UPDATE ON clipboard_entries BEGIN
            INSERT INTO clipboard_fts(clipboard_fts, rowid, content) VALUES('delete', old.id, old.content);
            INSERT INTO clipboard_fts(rowid, content) VALUES (new.id, new.content);
        END;
        "#,
    )?;
    
    // Create window position table
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS window_position (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            monitor_id TEXT NOT NULL,
            dock_position TEXT NOT NULL,
            x INTEGER NOT NULL,
            y INTEGER NOT NULL,
            width INTEGER NOT NULL,
            height INTEGER NOT NULL
        )
        "#,
        [],
    )?;
    
    // Create settings table
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )
        "#,
        [],
    )?;
    
    log::info!("Database initialized at {:?}", db_path);
    Ok(conn)
}

/// Get the default database path
pub fn get_database_path(app_data_dir: &Path) -> std::path::PathBuf {
    app_data_dir.join("clipboard_history.db")
}

/// Calculate content hash for deduplication
pub fn calculate_hash(content: &str) -> String {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    
    #[test]
    fn test_init_database() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        
        let conn = init_database(&db_path).unwrap();
        
        // Verify tables exist
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='clipboard_entries'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }
    
    #[test]
    fn test_calculate_hash() {
        let hash1 = calculate_hash("Hello World");
        let hash2 = calculate_hash("Hello World");
        let hash3 = calculate_hash("Different");
        
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}
