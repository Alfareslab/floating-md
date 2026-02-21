//! Database module - SQLite with FTS5 for clipboard history
//! 
//! This module provides:
//! - Schema management
//! - CRUD operations for clipboard entries
//! - Full-text search with Arabic support

mod schema;
mod queries;

pub use schema::*;
pub use queries::*;
