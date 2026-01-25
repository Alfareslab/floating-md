//! Database Queries
//! 
//! CRUD operations and search functionality for clipboard history.

use rusqlite::{Connection, Result, params};
use crate::input::{ClipboardEntry, ClipboardContentType};
use crate::window::{WindowPosition, DockPosition};
use super::schema::calculate_hash;

// ============ Clipboard Entry Queries ============

/// Insert a new clipboard entry
/// Returns the new entry ID, or None if the content already exists
pub fn insert_entry(conn: &Connection, entry: &ClipboardEntry) -> Result<Option<i64>> {
    let hash = calculate_hash(&entry.content);
    let content_type = match entry.content_type {
        ClipboardContentType::Text => "text",
        ClipboardContentType::Code => "code",
        ClipboardContentType::Markdown => "markdown",
    };
    
    // Try to insert, ignore if hash already exists
    let result = conn.execute(
        r#"
        INSERT OR IGNORE INTO clipboard_entries 
        (content, content_type, created_at, is_pinned, is_scrubbed, hash)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
        params![
            entry.content,
            content_type,
            entry.created_at,
            entry.is_pinned as i32,
            entry.is_scrubbed as i32,
            hash,
        ],
    )?;
    
    if result > 0 {
        Ok(Some(conn.last_insert_rowid()))
    } else {
        // Content already exists, update created_at to bump it to top
        conn.execute(
            "UPDATE clipboard_entries SET created_at = ?1 WHERE hash = ?2",
            params![entry.created_at, hash],
        )?;
        Ok(None)
    }
}

/// Get recent clipboard entries
pub fn get_recent_entries(conn: &Connection, limit: u32) -> Result<Vec<ClipboardEntry>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, content, content_type, created_at, is_pinned, is_scrubbed
        FROM clipboard_entries
        ORDER BY is_pinned DESC, created_at DESC
        LIMIT ?1
        "#,
    )?;
    
    let entries = stmt.query_map([limit], |row| {
        let content_type_str: String = row.get(2)?;
        let content_type = match content_type_str.as_str() {
            "code" => ClipboardContentType::Code,
            "markdown" => ClipboardContentType::Markdown,
            _ => ClipboardContentType::Text,
        };
        
        Ok(ClipboardEntry {
            id: Some(row.get(0)?),
            content: row.get(1)?,
            content_type,
            created_at: row.get(3)?,
            is_pinned: row.get::<_, i32>(4)? != 0,
            is_scrubbed: row.get::<_, i32>(5)? != 0,
        })
    })?;
    
    entries.collect()
}

/// Search clipboard entries using FTS5
pub fn search_entries(conn: &Connection, query: &str, limit: u32) -> Result<Vec<ClipboardEntry>> {
    // Escape special FTS5 characters and add prefix matching
    let search_query = format!("{}*", query.replace('"', "\"\""));
    
    let mut stmt = conn.prepare(
        r#"
        SELECT e.id, e.content, e.content_type, e.created_at, e.is_pinned, e.is_scrubbed
        FROM clipboard_entries e
        JOIN clipboard_fts f ON e.id = f.rowid
        WHERE clipboard_fts MATCH ?1
        ORDER BY e.is_pinned DESC, rank
        LIMIT ?2
        "#,
    )?;
    
    let entries = stmt.query_map(params![search_query, limit], |row| {
        let content_type_str: String = row.get(2)?;
        let content_type = match content_type_str.as_str() {
            "code" => ClipboardContentType::Code,
            "markdown" => ClipboardContentType::Markdown,
            _ => ClipboardContentType::Text,
        };
        
        Ok(ClipboardEntry {
            id: Some(row.get(0)?),
            content: row.get(1)?,
            content_type,
            created_at: row.get(3)?,
            is_pinned: row.get::<_, i32>(4)? != 0,
            is_scrubbed: row.get::<_, i32>(5)? != 0,
        })
    })?;
    
    entries.collect()
}

/// Get a single entry by ID
pub fn get_entry_by_id(conn: &Connection, id: i64) -> Result<Option<ClipboardEntry>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, content, content_type, created_at, is_pinned, is_scrubbed
        FROM clipboard_entries
        WHERE id = ?1
        "#,
    )?;
    
    let mut rows = stmt.query([id])?;
    
    if let Some(row) = rows.next()? {
        let content_type_str: String = row.get(2)?;
        let content_type = match content_type_str.as_str() {
            "code" => ClipboardContentType::Code,
            "markdown" => ClipboardContentType::Markdown,
            _ => ClipboardContentType::Text,
        };
        
        Ok(Some(ClipboardEntry {
            id: Some(row.get(0)?),
            content: row.get(1)?,
            content_type,
            created_at: row.get(3)?,
            is_pinned: row.get::<_, i32>(4)? != 0,
            is_scrubbed: row.get::<_, i32>(5)? != 0,
        }))
    } else {
        Ok(None)
    }
}

/// Pin or unpin an entry
pub fn set_entry_pinned(conn: &Connection, id: i64, pinned: bool) -> Result<usize> {
    conn.execute(
        "UPDATE clipboard_entries SET is_pinned = ?1 WHERE id = ?2",
        params![pinned as i32, id],
    )
}

/// Mark entry as scrubbed
pub fn set_entry_scrubbed(conn: &Connection, id: i64, scrubbed: bool) -> Result<usize> {
    conn.execute(
        "UPDATE clipboard_entries SET is_scrubbed = ?1 WHERE id = ?2",
        params![scrubbed as i32, id],
    )
}

/// Update entry content
pub fn update_entry_content(conn: &Connection, id: i64, content: &str) -> Result<usize> {
    let hash = calculate_hash(content);
    conn.execute(
        "UPDATE clipboard_entries SET content = ?1, hash = ?2 WHERE id = ?3",
        params![content, hash, id],
    )
}

/// Delete an entry
pub fn delete_entry(conn: &Connection, id: i64) -> Result<usize> {
    conn.execute("DELETE FROM clipboard_entries WHERE id = ?1", [id])
}

/// Delete old entries (keep only the most recent N)
pub fn prune_old_entries(conn: &Connection, keep_count: u32) -> Result<usize> {
    conn.execute(
        r#"
        DELETE FROM clipboard_entries 
        WHERE is_pinned = 0 
        AND id NOT IN (
            SELECT id FROM clipboard_entries 
            ORDER BY created_at DESC 
            LIMIT ?1
        )
        "#,
        [keep_count],
    )
}

// ============ Window Position Queries ============

/// Save window position
pub fn save_window_position(conn: &Connection, position: &WindowPosition) -> Result<usize> {
    let dock_position = match position.dock_position {
        DockPosition::Left => "left",
        DockPosition::Right => "right",
        DockPosition::Top => "top",
        DockPosition::Bottom => "bottom",
        DockPosition::Float => "float",
    };
    
    conn.execute(
        r#"
        INSERT OR REPLACE INTO window_position (id, monitor_id, dock_position, x, y, width, height)
        VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6)
        "#,
        params![
            position.monitor_id,
            dock_position,
            position.x,
            position.y,
            position.width,
            position.height,
        ],
    )
}

/// Load window position
pub fn load_window_position(conn: &Connection) -> Result<Option<WindowPosition>> {
    let mut stmt = conn.prepare(
        "SELECT monitor_id, dock_position, x, y, width, height FROM window_position WHERE id = 1",
    )?;
    
    let mut rows = stmt.query([])?;
    
    if let Some(row) = rows.next()? {
        let dock_str: String = row.get(1)?;
        let dock_position = match dock_str.as_str() {
            "left" => DockPosition::Left,
            "right" => DockPosition::Right,
            "top" => DockPosition::Top,
            "bottom" => DockPosition::Bottom,
            _ => DockPosition::Float,
        };
        
        Ok(Some(WindowPosition {
            monitor_id: row.get(0)?,
            dock_position,
            x: row.get(2)?,
            y: row.get(3)?,
            width: row.get(4)?,
            height: row.get(5)?,
        }))
    } else {
        Ok(None)
    }
}

// ============ Settings Queries ============

/// Set a setting value
pub fn set_setting(conn: &Connection, key: &str, value: &str) -> Result<usize> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        params![key, value],
    )
}

/// Get a setting value
pub fn get_setting(conn: &Connection, key: &str) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
    let mut rows = stmt.query([key])?;
    
    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::init_database;
    use tempfile::tempdir;
    
    fn create_test_db() -> Connection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        init_database(&db_path).unwrap()
    }
    
    #[test]
    fn test_insert_and_get_entry() {
        let conn = create_test_db();
        
        let entry = ClipboardEntry::new("Test content".to_string());
        let id = insert_entry(&conn, &entry).unwrap().unwrap();
        
        let retrieved = get_entry_by_id(&conn, id).unwrap().unwrap();
        assert_eq!(retrieved.content, "Test content");
    }
    
    #[test]
    fn test_duplicate_prevention() {
        let conn = create_test_db();
        
        let entry1 = ClipboardEntry::new("Same content".to_string());
        let id1 = insert_entry(&conn, &entry1).unwrap();
        
        let entry2 = ClipboardEntry::new("Same content".to_string());
        let id2 = insert_entry(&conn, &entry2).unwrap();
        
        assert!(id1.is_some());
        assert!(id2.is_none()); // Duplicate, returns None
    }
    
    #[test]
    fn test_pin_entry() {
        let conn = create_test_db();
        
        let entry = ClipboardEntry::new("Pin me".to_string());
        let id = insert_entry(&conn, &entry).unwrap().unwrap();
        
        set_entry_pinned(&conn, id, true).unwrap();
        
        let retrieved = get_entry_by_id(&conn, id).unwrap().unwrap();
        assert!(retrieved.is_pinned);
    }
}
