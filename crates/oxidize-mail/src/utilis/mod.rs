//! Utility functions and helpers for the Oxidize Mail application.
//!
//! This module contains utility functions that provide core functionality
//! used throughout the application. These utilities handle resource management,
//! styling, and other common operations that support the main application logic.
//!
//! # Modules
//!
//! * `register_resources` - GResource registration for embedded application resources
//! * `style` - CSS styling and theme management utilities
//!
//! # Example
//!
//! ```rust
//! use crate::utilis::{register_resources, style};
//! use oxidize_mail_types::UserConfig;
//! use std::cell::RefCell;
//! use std::rc::Rc;
//!
//! // Register application resources at startup
//! register_resources::register_resources();
//!
//! // Load CSS styles
//! style::load_css();
//!
//! // Apply theme to a window
//! let config = Rc::new(RefCell::new(UserConfig::default()));
//! // style::apply_theme(&window, config);
//! ```

pub mod register_resources;
pub mod style;
