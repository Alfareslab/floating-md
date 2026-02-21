//! Window module - Handles WS_EX_NOACTIVATE and window management
//! 
//! This module implements the core functionality to prevent focus stealing
//! when the floating bar is clicked.

mod noactivate;
mod docking;

pub use noactivate::*;
pub use docking::*;
