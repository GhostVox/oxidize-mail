//! User interface components for the Oxidize Mail application.
//!
//! This module contains all the UI-related code organized into separate submodules
//! for different areas of the application interface. Each module handles a specific
//! part of the user interface and provides functions to create and manage the
//! corresponding widgets.
//!
//! # Modules
//!
//! * `header_bar` - Application header bar with title and window controls
//! * `left_pane` - Folder sidebar and email list interface
//! * `middle_pane` - Middle pane functionality (currently unused)
//! * `right_pane` - Right pane functionality (currently unused)
//! * `settings_dialog` - Settings dialog window and preference controls
//!
//! # Example
//!
//! ```rust
//! use crate::ui::{header_bar, left_pane, settings_dialog};
//! use oxidize_mail_types::UserConfig;
//! use std::cell::RefCell;
//! use std::rc::Rc;
//!
//! let config = Rc::new(RefCell::new(UserConfig::default()));
//!
//! // Create header bar
//! let (header, title_rc) = header_bar::create_headerbar(config.clone());
//!
//! // Create left pane with folder navigation
//! // let (main_paned, email_container) = left_pane::create_left_pane(...);
//!
//! // Show settings dialog when needed
//! // let settings_window = settings_dialog::show_settings_dialog(&window, config);
//! ```

pub mod header_bar;
pub mod left_pane;
pub mod middle_pane;
pub mod right_pane;
pub mod settings_dialog;
