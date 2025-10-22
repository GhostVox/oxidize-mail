use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::{
    glib, Application, ApplicationWindow, Box, Orientation, Paned, PolicyType, ScrolledWindow,
};
use oxidize_mail_types::UserConfig;
mod ui;
mod utilis;
use std::cell::RefCell;
use std::rc::Rc;
use ui::{header_bar, left_pane, settings_dialog};
use utilis::{register_resources, style};

const APP_ID: &str = "com.oxidize.mail";

fn main() -> glib::ExitCode {
    // Register resources first
    register_resources::register_resources();

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

/// Builds the main UI for the application, ment to be used as the `activate` signal handler.
///
/// # Arguments
///
/// * `app` - Application instance to build the UI for.
///
/// # Examples
///
/// ```
/// app.connect_activate(build_ui);
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

    // --- UI Layout ---
    let (main_paned, email_list_container) =
        left_pane::create_left_pane(settings_rc.clone(), title_rc.clone(), &window);
    let content_paned = Paned::new(Orientation::Horizontal);

    // Middle Pane (now using the container we created)
    content_paned.set_start_child(Some(&email_list_container));
    content_paned.set_resize_start_child(true);
    content_paned.set_shrink_start_child(false);

    // Right Pane
    //content_paned.set_end_child(Some(&email_viewer));
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

//FIXME: Make this dynamic using webkit to render actual email content

/// Creates the email viewer pane.
///
/// # Arguments
/// * `selected_email` - The email to display in the viewer.
///
/// # Examples
///
/// ```
/// let email_viewer = create_email_viewer(selected_email);
/// ```
fn create_email_viewer_widgets() -> (Box, Box, Box) {
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

    content.append(&header_box);

    //TODO: fill this box with webkit
    let viewer_box = Box::new(Orientation::Vertical, 0);

    content.append(&header_box);
    content.append(&viewer_box);
    scrolled.set_child(Some(&content));
    viewer.append(&scrolled);
    (viewer, header_box, viewer_box)
}
