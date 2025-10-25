//! Oxidize Mail - A modern email client built with Rust and GTK4.
//!
//! This is the main application entry point for Oxidize Mail, a cross-platform
//! email client built using Rust, GTK4, and WebKit. The application provides
//! a modern, responsive interface for managing email with support for multiple
//! accounts, folders, and customizable themes.
//!
//! # Features
//!
//! - Modern GTK4-based user interface
//! - WebKit-powered email content rendering
//! - Customizable themes (Light, Dark, System)
//! - Folder-based email organization
//! - Settings dialog for user preferences
//! - Resource bundling for optimal performance
//!
//! # Architecture
//!
//! The application follows a modular architecture:
//! - UI components are organized in the `ui` module
//! - Utility functions are in the `utilis` module
//! - Type definitions are provided by `oxidize_mail_types`
//! - Email parsing is handled by `oxidize_mail_parser`
//! - Data storage uses `oxidize_mail_storage`
//!
//! # Example Usage
//!
//! The application is launched from the command line:
//!
//! ```bash
//! cargo run
//! ```
//!
//! This initializes the GTK application, loads resources, applies styling,
//! and presents the main interface to the user.

use gtk4::{
    glib, Application, ApplicationWindow, Box, Orientation, Paned, PolicyType, ScrolledWindow,
};
use gtk4::{glib::clone, prelude::*};
use oxidize_mail_types::UserConfig;
use webkit6::{prelude::WebViewExt, WebView};
mod startup;
mod ui;
mod utilis;
use startup::startup;
use std::cell::RefCell;
use std::rc::Rc;
use ui::{header_bar, left_pane, settings_dialog};
use utilis::{register_resources, style};

/// Application identifier used for GTK Application registration.
///
/// This constant defines the unique application ID that GTK uses to identify
/// the Oxidize Mail application. It follows the reverse domain name convention
/// and is used for application registration, D-Bus integration, and desktop
/// environment integration.
///
/// The ID ensures that the application can be uniquely identified by the
/// desktop environment and prevents conflicts with other applications.
const APP_ID: &str = "com.oxidize.mail";

/// Main application entry point.
///
/// This function initializes and runs the Oxidize Mail application. It performs
/// the following initialization steps:
/// 1. Registers embedded GLib resources
/// 2. Creates a GTK Application instance with the application ID
/// 3. Connects the UI builder to the application activation signal
/// 4. Runs the GTK main event loop
///
/// # Returns
///
/// A `glib::ExitCode` indicating the application's exit status:
/// * `SUCCESS` (0) - Application ran and exited normally
/// * Other codes - Application encountered an error or was terminated
///
/// # Examples
///
/// The application is typically launched from the command line:
/// ```bash
/// cargo run
/// ```
///
/// # Panics
///
/// May panic if:
/// * GTK initialization fails
/// * Required system resources are unavailable
/// * Application ID registration conflicts with another running instance
fn main() -> glib::ExitCode {
    // Register resources first
    env_logger::init();
    register_resources::register_resources();
    let coreService = startup();

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

/// Builds the main UI for the application, meant to be used as the `activate` signal handler.
///
/// This function creates the complete application UI including the header bar, main content
/// panes, email viewer, and settings dialog. It sets up the layout with proper sizing,
/// applies CSS themes, and establishes all necessary signal connections for user interactions.
///
/// # Arguments
///
/// * `app` - The GTK4 Application instance to build the UI for.
///
/// # Examples
///
/// ```rust
/// use gtk4::{Application, prelude::*};
///
/// let app = Application::builder()
///     .application_id("com.oxidize.mail")
///     .build();
/// app.connect_activate(build_ui);
/// app.run();
/// ```
fn build_ui(app: &Application) {
    style::load_css();
    let settings_rc = Rc::new(RefCell::new(UserConfig::load()));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Oxidize Mail")
        .default_width(1400)
        .default_height(900)
        .build();

    style::apply_theme(&window, settings_rc.clone());

    // Creates the Header Widget
    let (header, title_rc) = header_bar::create_headerbar(settings_rc.clone());
    window.set_titlebar(Some(&header));

    //RIGHT PANE
    let (email_viewer, email_header_box, webkit_box_box) = create_email_viewer_widgets();

    let email_header_rc = Rc::new(RefCell::new(email_header_box));
    let webkit_box_rc = Rc::new(RefCell::new(webkit_box_box));

    //MIDDLE PANE
    //TODO: move email list creation to middle_pane module and call it her

    // --- UI Layout ---
    let (main_paned, email_list_container) = left_pane::create_left_pane(
        settings_rc.clone(),
        title_rc.clone(),
        &window,
        email_header_rc.clone(),
        webkit_box_rc.clone(),
    );
    let content_paned = Paned::new(Orientation::Horizontal);

    // Middle Pane (now using the container we created)
    content_paned.set_start_child(Some(&email_list_container));
    content_paned.set_resize_start_child(true);
    content_paned.set_shrink_start_child(false);

    // Right Pane
    content_paned.set_end_child(Some(&email_viewer));
    content_paned.set_resize_end_child(true);
    content_paned.set_shrink_end_child(false);

    main_paned.set_end_child(Some(&content_paned));
    main_paned.set_position(220);
    content_paned.set_position(450);

    window.set_child(Some(&main_paned));

    // Save settings on close
    window.connect_close_request(clone!(
        #[strong]
        settings_rc,
        move |_| {
            settings_rc.borrow().save();
            glib::Propagation::Proceed
        }
    ));

    window.present();
}

/// Creates the email viewer pane widgets including the main container, header box, and WebKit WebView.
///
/// This function constructs a vertical box container that holds an email header section
/// and a WebKit WebView for rendering email content. The WebView is configured with
/// proper expansion settings and CSS classes for styling.
///
/// # Returns
///
/// A tuple containing:
/// * `Box` - The main email viewer container
/// * `Box` - The email header information box
/// * `WebView` - The WebKit WebView for displaying email content
///
/// # Examples
///
/// ```rust
/// use gtk4::{Box, prelude::*};
/// use webkit6::WebView;
///
/// let (email_viewer, email_header_box, webkit_webview) = create_email_viewer_widgets();
/// // Use the returned widgets to build your email display interface
/// ```
fn create_email_viewer_widgets() -> (Box, Box, WebView) {
    let viewer = Box::new(Orientation::Vertical, 0);
    viewer.set_vexpand(true);
    viewer.set_hexpand(true);
    viewer.add_css_class("email-viewer");

    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(PolicyType::Automatic, PolicyType::Automatic);
    scrolled.set_vexpand(true);

    let content = Box::new(Orientation::Vertical, 0);

    // Email header info
    let header_box = Box::new(Orientation::Vertical, 4);
    header_box.set_margin_start(20);
    header_box.set_margin_end(20);
    header_box.set_margin_top(20);
    header_box.set_margin_bottom(20);
    header_box.add_css_class("email-header-box");
    content.append(&header_box);

    //TODO: on launch display a welcome message or collapse the box until an email is selected
    // WebKit WebView for email content
    let webview = WebView::new();
    webview.set_vexpand(true);
    webview.set_hexpand(true);
    webview.add_css_class("email-webview");
    webview.load_html(
        "<h1>Welcome to Oxidize Mail!</h1><p>This is a sample email content rendered using WebKit.</p>",
        None,
    );

    content.append(&webview);

    scrolled.set_child(Some(&content));
    viewer.append(&scrolled);

    (viewer, header_box, webview)
}
