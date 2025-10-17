use fake::faker::internet::en::SafeEmail;
use fake::faker::lorem::en::Sentence;
use fake::Fake;
use fake::Faker;
// use fake::faker::name::en::Name;
use gtk4::glib::clone;
use gtk4::{prelude::*, Button, Settings};
use std::collections::HashMap;
mod config;
use gtk4::{
    gdk::Display, gio, glib, Application, ApplicationWindow, Box, CssProvider, HeaderBar, Label,
    ListBox, Orientation, Paned, PolicyType, ScrolledWindow,
};

use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "com.oxidize.mail";

fn main() -> glib::ExitCode {
    // Register resources first
    register_resources();

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

/// Register the GResources for the application.
///
/// # Examples
///
/// ```
/// register_resources();
/// ```
fn register_resources() {
    let resource_bytes = glib::Bytes::from_static(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/oxidize-mail.gresource"
    )));
    let resource = gio::Resource::from_data(&resource_bytes).expect("Failed to load GResource");
    gio::resources_register(&resource);
}

/// Applies the theme to the given window based on the configuration.
///
/// # Arguments
///
/// * `window` - Application window to apply the theme to.
/// * `config` - AppConfig instance containing user preferences.
///
/// # Examples
///
/// ```
/// apply_theme(&window, settings_rc.clone());
/// ```
fn apply_theme(window: &ApplicationWindow, config: Rc<RefCell<config::AppConfig>>) {
    let should_use_dark = match config.borrow().get_preferred_color_scheme() {
        config::ColorScheme::Light => false,
        config::ColorScheme::Dark => true,
        config::ColorScheme::System => {
            // Check GTK system preference
            let gtk_settings = Settings::default().expect("Could not get GTK settings");
            gtk_settings.property::<bool>("gtk-application-prefer-dark-theme")
        }
    };

    // Add or remove the "dark" CSS class
    if should_use_dark {
        window.add_css_class("dark");
    } else {
        window.remove_css_class("dark");
    }
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
    load_css();
    let settings_rc = Rc::new(RefCell::new(config::AppConfig::load()));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Oxidize Mail")
        .default_width(1400)
        .default_height(900)
        .build();

    apply_theme(&window, settings_rc.clone());
    // Header bar setup
    let header = HeaderBar::new();
    header.set_show_title_buttons(true);
    let title = Label::new(Some(&settings_rc.borrow().get_selected_folder()));
    title.add_css_class("title");
    let title_rc = Rc::new(RefCell::new(title.clone()));
    header.set_title_widget(Some(&title));
    window.set_titlebar(Some(&header));

    // Email data generation
    let emails = Rc::new(RefCell::new(generate_sample_emails()));

    // 1. Create the email list widgets, getting a reference to the ListBox
    let (email_viewer, email_header_box, email_viewer_box) = create_email_viewer_widgets();
    let email_viewer_rc = Rc::new(RefCell::new(email_viewer));

    let (email_list_container, email_listbox) = create_email_list_widgets();
    let email_listbox_rc = Rc::new(RefCell::new(email_listbox));

    // 2. Populate the list with initial data
    populate_email_list(
        &email_listbox_rc.borrow(),
        &settings_rc.borrow().get_selected_folder(),
        &emails.borrow(),
    );

    // 3. Create the folder sidebar, passing the email ListBox reference to it
    let folder_sidebar = create_folder_sidebar(
        title_rc.clone(),
        settings_rc.clone(),
        emails.clone(),
        email_listbox_rc, // Pass the reference
    );

    // --- UI Layout ---
    let main_paned = Paned::new(Orientation::Horizontal);
    let content_paned = Paned::new(Orientation::Horizontal);

    // Left Pane
    main_paned.set_start_child(Some(&folder_sidebar));
    main_paned.set_resize_start_child(false);
    main_paned.set_shrink_start_child(false);

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

/// Loads the CSS from resources and applies it to the application display.
///
/// # Examples
///
/// ```
/// load_css();
/// ```
fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/com/oxidize/mail/css/style.css");
    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Creates the folder sidebar with folder sections and items;
///
/// # Arguments
///
/// * `title_label` - Label of active inbox, to be updated on folder selection. associated with the
/// header bar.
/// * `settings` - Appconfig instance of user preferences. Will be updated on folder selection.
/// * `emails` - HashMap of email data, used to repopulate the email list on folder selection.
/// * `email_listbox` - Email list box. The ListBox to be repopulated when a folder is selected.
///
/// # Examples
///
/// ```
/// let folder_sidebar = create_folder_sidebar(
///     title_rc.clone(),
///     settings_rc.clone(),
///     emails.clone(),
///     email_listbox_rc, // Pass the reference
/// );
/// ```
fn create_folder_sidebar(
    title_label: Rc<RefCell<Label>>,
    settings: Rc<RefCell<config::AppConfig>>,
    emails: Rc<RefCell<HashMap<String, Vec<Email>>>>,
    email_listbox: Rc<RefCell<ListBox>>, // MODIFIED: Takes the ListBox now
) -> Box {
    let sidebar = Box::new(Orientation::Vertical, 0);
    sidebar.set_width_request(220);
    sidebar.set_vexpand(true);
    sidebar.add_css_class("navigation-sidebar");

    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(PolicyType::Never, PolicyType::Automatic);
    scrolled.set_vexpand(true);

    let listbox = ListBox::new();

    // Hardcoded folder sections (can be dynamic later)
    let sections = vec![
        ("Favorites", vec!["📥 All Inboxes", "📧 Bret637@gmail.com"]),
        (
            "iCloud",
            vec![
                "📥 Inbox",
                "📤 Sent",
                "✏️ Drafts",
                "📁 Junk",
                "🗑️ Trash",
                "📦 Archive",
            ],
        ),
        (
            "Smart Mailboxes",
            vec![
                "🔵 Important",
                "⭐ Drafts",
                "📤 Sent",
                "📁 Junk",
                "🗑️ Trash",
            ],
        ),
    ];

    for (section_name, folders) in sections {
        let header = Label::new(Some(section_name));
        header.set_halign(gtk4::Align::Start);
        header.set_margin_start(12);
        header.set_margin_top(12);
        header.set_margin_bottom(4);
        header.add_css_class("caption");
        header.add_css_class("dim-label");
        listbox.append(&header);

        for folder in folders {
            let label = Label::new(Some(folder));
            label.set_halign(gtk4::Align::Start);
            label.add_css_class("folder-item");
            listbox.append(&label);
        }
    }

    // Connect row-selected signal to update title and repopulate the email list
    listbox.connect_row_selected(clone!(
        #[strong]
        title_label,
        #[strong]
        settings,
        #[strong]
        emails,
        #[strong]
        email_listbox,
        move |_, row| {
            if let Some(row) = row {
                if let Some(label) = row.child().and_then(|child| child.downcast::<Label>().ok()) {
                    let folder_name = label.text();
                    // Ensure we're not clicking a section header
                    if !folder_name.is_empty() && !label.has_css_class("dim-label") {
                        // Update header title and settings
                        title_label.borrow_mut().set_text(&folder_name);
                        settings.borrow_mut().update_selected_folder(&folder_name);

                        // ** THE FIX **
                        // Call the function to repopulate the existing ListBox
                        populate_email_list(
                            &email_listbox.borrow(),
                            &folder_name,
                            &emails.borrow(),
                        );
                    }
                }
            }
        }
    ));

    scrolled.set_child(Some(&listbox));
    sidebar.append(&scrolled);
    //TODO: Add a floating settings modal when clicked
    let settings_button = create_settings_button();
    sidebar.append(&settings_button);
    sidebar
}

/// Renders teh settings button in the folder sidebar.
///
/// # Examples
///
/// ```
/// let settings_button = create_settings_button();
/// ```
fn create_settings_button() -> Button {
    let settings_button = Button::builder()
        .label("  Settings")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .build();
    settings_button.add_css_class("settings-button");
    settings_button.set_halign(gtk4::Align::Start);
    settings_button
}

/// Creates the email list container and ListBox.
///
/// # Examples
///
/// ```
/// let (email_list_container, email_listbox) = create_email_list_widgets();
/// ```
fn create_email_list_widgets() -> (Box, ListBox) {
    let list_container = Box::new(Orientation::Vertical, 0);
    list_container.set_vexpand(true);
    list_container.set_hexpand(true);
    list_container.add_css_class("email-list");

    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(PolicyType::Never, PolicyType::Automatic);
    scrolled.set_vexpand(true);

    let listbox = ListBox::new();

    // listbox.connect_row_selected(clone!(
    //     #[strong]
    //     emails,
    //     #[strong]
    //     email_viewer_box,
    //     move |_, row| {
    //         //TODO: Update email viewer with selected email content
    //     }
    // ));
    scrolled.set_child(Some(&listbox));
    list_container.append(&scrolled);
    (list_container, listbox)
}

/// Populates the given ListBox with email rows for teh specified folder.
///
/// # Arguments
///
/// * `listbox` - ListBox to repopulate with correspoinding emails.
/// * `folder_name` - Name of the folder to get emails for.
/// * `emails` - Email HashMap containing all email data.
///
/// # Examples
///
/// ```
/// populate_email_list(
///        &email_listbox_rc.borrow(),
///        &settings_rc.borrow().get_selected_folder(),
///        &emails.borrow(),
/// );
/// ```
fn populate_email_list(listbox: &ListBox, folder_name: &str, emails: &HashMap<String, Vec<Email>>) {
    // Clear existing rows
    while let Some(child) = listbox.first_child() {
        listbox.remove(&child);
    }

    // Get the emails for the selected folder and create new rows
    if let Some(email_list) = emails.get(folder_name) {
        for (i, e) in email_list.iter().enumerate() {
            let email_row = Box::new(Orientation::Horizontal, 0);
            email_row.set_margin_start(8);
            email_row.set_margin_end(8);
            email_row.set_margin_top(4);
            email_row.set_margin_bottom(4);
            email_row.add_css_class("email-row");

            // Hardcoded selection for demonstration
            if i == 7 {
                email_row.add_css_class("selected");
            }

            let content_box = Box::new(Orientation::Vertical, 2);
            content_box.set_hexpand(true);

            if !e.sender.is_empty() {
                let sender_label = Label::new(Some(&e.sender));
                sender_label.set_halign(gtk4::Align::Start);
                sender_label.add_css_class("email-sender");
                sender_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
                content_box.append(&sender_label);
            }

            let subject_label = Label::new(Some(&e.subject));
            subject_label.set_halign(gtk4::Align::Start);
            subject_label.add_css_class("email-subject");
            subject_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);

            let preview_label = Label::new(Some(&e.preview));
            preview_label.set_halign(gtk4::Align::Start);
            preview_label.add_css_class("email-preview");
            preview_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);

            content_box.append(&subject_label);
            content_box.append(&preview_label);

            let time_label = Label::new(Some(&e.time));
            time_label.set_halign(gtk4::Align::End);
            time_label.set_valign(gtk4::Align::Start);
            time_label.add_css_class("email-time");

            email_row.append(&content_box);
            email_row.append(&time_label);

            listbox.append(&email_row);
        }
    }
}

fn generate_sample_emails() -> HashMap<String, Vec<Email>> {
    let mut emails: HashMap<String, Vec<Email>> = HashMap::new();
    let folders = vec![
        "📥 Inbox",
        "📤 Sent",
        "✏️ Drafts",
        "📁 Junk",
        "🗑️ Trash",
        "📦 Archive",
    ];

    for f in folders {
        for _ in 0..20 {
            // Added more emails for better scrolling demo
            let subject: String = Sentence(5..10).fake();
            let sender_email: String = SafeEmail().fake();
            let sender = format!("Inbox - {}", sender_email);
            let body: String = Sentence(50..100).fake();
            let preview: String = body.chars().take(60).collect();
            let time = format!(
                "{}:{:02} {}",
                (1..12).fake::<u8>(),
                (0..59).fake::<u8>(),
                if Faker.fake::<bool>() { "AM" } else { "PM" }
            );

            emails
                .entry(f.to_string())
                .or_insert_with(Vec::new)
                .push(Email {
                    subject,
                    sender,
                    preview,
                    time,
                    body,
                });
        }
    }
    emails
}

fn populate_email_viewer() {

    // let subject = Label::new(Some(&selected_email.subject));
    // subject.set_halign(gtk4::Align::Start);
    // subject.add_css_class("viewer-subject");

    // let from = Label::new(Some(&selected_email.sender));
    // from.set_halign(gtk4::Align::Start);
    // from.add_css_class("viewer-header");

    // let time = Label::new(Some(&selected_email.time));
    // from.set_halign(gtk4::Align::Start);
    // from.add_css_class("viewer-header");

    // //WARNING: i think this is supposed to be a button or link
    // let reply_to = Label::new(Some("Reply-To: Best Buy"));
    // reply_to.set_halign(gtk4::Align::Start);
    // reply_to.add_css_class("viewer-header");
    // header_box.append(&subject);
    // header_box.append(&from);
    // header_box.append(&reply_to);
    // header_box.append(&time);
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

struct Email {
    subject: String,
    sender: String,
    preview: String,
    time: String,
    body: String,
}

impl Email {
    fn new(subject: String, sender: String, preview: String, time: String, body: String) -> Self {
        Self {
            subject,
            sender,
            preview,
            time,
            body,
        }
    }
}
