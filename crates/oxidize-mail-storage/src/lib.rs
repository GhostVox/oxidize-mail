//! Email storage and database management for the Oxidize Mail application.
//!
//! This crate provides database connection management and storage operations
//! for the Oxidize Mail email client. It uses SQLite as the underlying database
//! engine and provides a high-level interface for email data persistence.
//!
//! # Features
//!
//! - SQLite database connection management
//! - Email data storage and retrieval operations
//! - Configuration-driven database setup
//! - Error handling for database operations
//!
//! # Example
//!
//! ```rust
//! use oxidize_mail_storage::DB;
//! use oxidize_mail_types::AppConfig;
//!
//! let config = AppConfig::default();
//! match DB::new(&config) {
//!     Ok(db) => {
//!         println!("Database connection established");
//!         // Perform database operations...
//!     }
//!     Err(e) => eprintln!("Failed to connect to database: {}", e),
//! }
//! ```
//!
//! # Note
//!
//! This crate is currently in early development and contains placeholder
//! functions that will be replaced with actual storage implementation.

mod database;
mod queries;

pub use database::DB;
