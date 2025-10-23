//! Type definitions and data structures for the Oxidize Mail application.
//!
//! This crate provides all the core data types, configuration structures,
//! and email representation used throughout the Oxidize Mail email client.
//! It serves as the foundation for data exchange between different components
//! of the application.
//!
//! # Modules
//!
//! * `app_config` - Application-wide configuration settings
//! * `parsed_email` - Email data representation after parsing
//! * `user_config` - User preferences and settings
//!
//! # Types
//!
//! * `AppConfig` - Core application configuration
//! * `ParsedEmail` - Structured representation of parsed email data
//! * `UserConfig` - User preferences and session settings
//! * `ColorScheme` - Theme and color scheme preferences
//!
//! # Example
//!
//! ```rust
//! use oxidize_mail_types::{UserConfig, ColorScheme, ParsedEmail};
//!
//! // Create user configuration
//! let mut config = UserConfig::default();
//! config.update_color_scheme(ColorScheme::Light);
//! config.update_selected_folder("📤 Sent");
//!
//! // Work with parsed email data
//! let email = ParsedEmail {
//!     subject: Some("Hello World".to_string()),
//!     from: Some("sender@example.com".to_string()),
//!     to: Some("recipient@example.com".to_string()),
//!     body_text: Some("Email content...".to_string()),
//!     body_html: None,
//! };
//! ```

mod app_config;
mod parsed_email;
mod user_config;

pub use app_config::AppConfig;
pub use parsed_email::ParsedEmail;
pub use user_config::{ColorScheme, UserConfig};
