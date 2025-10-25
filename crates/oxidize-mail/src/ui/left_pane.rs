use crate::settings_dialog;
use fake::faker::internet::en::SafeEmail;
use fake::faker::lorem::en::Sentence;
use fake::Fake;
use fake::Faker;
use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::Paned;
use gtk4::{
    ApplicationWindow, Box, Button, Label, ListBox, Orientation, PolicyType, ScrolledWindow,
};
use oxidize_mail_types::ParsedEmail;
use oxidize_mail_types::UserConfig;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use webkit6::prelude::WebViewExt;
use webkit6::WebView;

/// Creates a settings button widget for the folder sidebar with dialog functionality.
///
/// This function creates a styled button that displays the settings icon and label.
/// When clicked, it opens the settings dialog window. The button is configured with
/// proper margins and CSS classes for consistent styling within the sidebar.
///
/// # Arguments
///
/// * `window` - The parent ApplicationWindow that will own the settings dialog
/// * `config` - Shared reference to the UserConfig for managing application settings
///
/// # Returns
///
/// A configured Button widget with click handler attached
///
/// # Examples
///
/// ```rust
/// use gtk4::{ApplicationWindow, Button, prelude::*};
/// use oxidize_mail_types::UserConfig;
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// let window = ApplicationWindow::new();
/// let config = Rc::new(RefCell::new(UserConfig::default()));
/// let settings_button = create_settings_button(&window, config);
/// ```
pub fn create_settings_button(
    window: &ApplicationWindow,
    config: Rc<RefCell<UserConfig>>,
) -> Button {
    let settings_button = Button::builder()
        .label("⚙️  Settings")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .build();
    settings_button.add_css_class("settings-button");
    settings_button.set_halign(gtk4::Align::Start);

    // Connect click signal to show the settings window
    let window_weak = window.downgrade();
    settings_button.connect_clicked(clone!(
        #[strong]
        config,
        move |_| {
            if let Some(window) = window_weak.upgrade() {
                // Show the settings window (returns Window, not Dialog)
                let _settings_window =
                    settings_dialog::show_settings_dialog(&window, config.clone());
            }
        }
    ));

    settings_button
}

/// Creates the left pane containing folder sidebar and email list.
///
/// This function constructs the complete left pane of the email client interface,
/// including the folder navigation sidebar and email list container. It generates
/// sample email data, sets up the email list widgets with proper event handling,
/// and creates the folder sidebar with navigation functionality.
///
/// # Arguments
///
/// * `settings_rc` - Shared reference to UserConfig for persistent settings
/// * `title_rc` - Shared reference to the header title label for updates
/// * `window` - Parent ApplicationWindow for dialog creation
/// * `email_header_rc` - Shared reference to email header display container
/// * `webkit_box_rc` - Shared reference to WebKit WebView for email content
///
/// # Returns
///
/// A tuple containing:
/// * `Paned` - The main horizontal paned widget containing folder sidebar
/// * `Box` - The email list container widget
///
/// # Examples
///
/// ```rust
/// use gtk4::{ApplicationWindow, Label, Box, Paned, prelude::*};
/// use webkit6::WebView;
/// use oxidize_mail_types::UserConfig;
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// let window = ApplicationWindow::new();
/// let settings = Rc::new(RefCell::new(UserConfig::default()));
/// let title = Rc::new(RefCell::new(Label::new(Some("Inbox"))));
/// let header_box = Rc::new(RefCell::new(Box::new(gtk4::Orientation::Vertical, 0)));
/// let webview = Rc::new(RefCell::new(WebView::new()));
///
/// let (main_paned, email_container) = create_left_pane(
///     settings, title, &window, header_box, webview
/// );
/// ```
//TODO: refactor so that create_email_list_widgets is not called here
pub fn create_left_pane(
    settings_rc: Rc<RefCell<UserConfig>>,
    title_rc: Rc<RefCell<Label>>,
    window: &ApplicationWindow,
    email_header_rc: Rc<RefCell<Box>>,
    webkit_box_rc: Rc<RefCell<WebView>>,
) -> (Paned, Box) {
    let emails = Rc::new(RefCell::new(generate_sample_emails()));
    let (email_list_container, email_listbox) = create_email_list_widgets(
        emails.clone(),
        email_header_rc.clone(),
        webkit_box_rc.clone(),
        title_rc.clone(),
    );

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
        &window,
    );

    let main_paned = Paned::new(Orientation::Horizontal);
    // Left Pane
    main_paned.set_start_child(Some(&folder_sidebar));
    main_paned.set_resize_start_child(false);
    main_paned.set_shrink_start_child(false);
    (main_paned, email_list_container)
}

/// Creates the email list container and ListBox with selection handling.
///
/// This function constructs a scrollable container that holds a ListBox for displaying
/// email items. It sets up row selection handling that updates the email viewer when
/// an email is selected. The container is configured with proper expansion settings
/// and CSS classes for styling.
///
/// # Arguments
///
/// * `emails` - Shared HashMap containing email data organized by folder
/// * `email_header_rc` - Shared reference to the email header display box
/// * `webkit_box_rc` - Shared reference to the WebKit WebView for email content
/// * `title_rc` - Shared reference to the title label for folder name display
///
/// # Returns
///
/// A tuple containing:
/// * `Box` - The main email list container with scrolling
/// * `ListBox` - The ListBox widget for email items
///
/// # Examples
///
/// ```rust
/// use gtk4::{Box, ListBox, prelude::*};
/// use webkit6::WebView;
/// use std::cell::RefCell;
/// use std::rc::Rc;
/// use std::collections::HashMap;
///
/// let emails = Rc::new(RefCell::new(HashMap::new()));
/// let email_header = Rc::new(RefCell::new(Box::new(gtk4::Orientation::Vertical, 0)));
/// let webkit_view = Rc::new(RefCell::new(WebView::new()));
/// let title_label = Rc::new(RefCell::new(gtk4::Label::new(None)));
///
/// let (container, listbox) = create_email_list_widgets(
///     emails, email_header, webkit_view, title_label
/// );
/// ```
pub fn create_email_list_widgets(
    emails: Rc<RefCell<HashMap<String, Vec<ParsedEmail>>>>,
    email_header_rc: Rc<RefCell<Box>>,
    webkit_box_rc: Rc<RefCell<WebView>>,
    title_rc: Rc<RefCell<Label>>,
) -> (Box, ListBox) {
    let list_container = Box::new(Orientation::Vertical, 0);
    list_container.set_vexpand(true);
    list_container.set_hexpand(true);
    list_container.add_css_class("email-list");

    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(PolicyType::Never, PolicyType::Automatic);
    scrolled.set_vexpand(true);

    let listbox = ListBox::new();

    listbox.connect_row_selected(clone!(
        #[strong]
        emails,
        #[strong]
        email_header_rc,
        #[strong]
        webkit_box_rc,
        move |_, row| {
            if let Some(row) = row {
                let index = row.index() as usize;
                let folder_name = title_rc.borrow().text().to_string();
                if let Some(email_list) = emails.borrow().get(&folder_name) {
                    if index < email_list.len() {
                        let selected_email = &email_list[index];
                        populate_email_viewer(
                            email_header_rc.clone(),
                            webkit_box_rc.clone(),
                            selected_email,
                        );
                    }
                    //TODO: switch the CSS class for selected row
                    //remove "selected" class from current selected row
                }
            }
        }
    ));
    scrolled.set_child(Some(&listbox));
    list_container.append(&scrolled);
    (list_container, listbox)
}

/// Creates the folder sidebar with hierarchical folder sections and navigation items.
///
/// This function constructs a scrollable sidebar containing organized folder sections
/// such as "Favorites", "iCloud", and "Smart Mailboxes". Each section contains
/// clickable folder items that update the main email list when selected. The sidebar
/// includes a settings button at the bottom and handles folder selection logic.
///
/// # Arguments
///
/// * `title_label` - Shared reference to the header title label that gets updated when folders are selected
/// * `settings` - Shared reference to UserConfig for persisting user preferences and selected folder
/// * `emails` - Shared HashMap containing all email data organized by folder name
/// * `email_listbox` - Shared reference to the main email ListBox that gets repopulated on folder changes
/// * `window` - Parent ApplicationWindow for the settings dialog
///
/// # Returns
///
/// A Box widget containing the complete folder sidebar with all sections and functionality
///
/// # Examples
///
/// ```rust
/// use gtk4::{Box, Label, ListBox, ApplicationWindow, prelude::*};
/// use oxidize_mail_types::UserConfig;
/// use std::cell::RefCell;
/// use std::rc::Rc;
/// use std::collections::HashMap;
///
/// let title_label = Rc::new(RefCell::new(Label::new(Some("Inbox"))));
/// let settings = Rc::new(RefCell::new(UserConfig::default()));
/// let emails = Rc::new(RefCell::new(HashMap::new()));
/// let email_listbox = Rc::new(RefCell::new(ListBox::new()));
/// let window = ApplicationWindow::new();
///
/// let folder_sidebar = create_folder_sidebar(
///     title_label,
///     settings,
///     emails,
///     email_listbox,
///     &window,
/// );
/// ```
pub fn create_folder_sidebar(
    title_label: Rc<RefCell<Label>>,
    settings: Rc<RefCell<UserConfig>>,
    emails: Rc<RefCell<HashMap<String, Vec<ParsedEmail>>>>,
    email_listbox: Rc<RefCell<ListBox>>, // MODIFIED: Takes the ListBox now
    window: &ApplicationWindow,
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
    let settings_button = create_settings_button(window, settings.clone());
    sidebar.append(&settings_button);
    sidebar
}

/// Populates the given ListBox with email rows for the specified folder.
///
/// This function clears all existing email rows from the ListBox and repopulates it
/// with emails from the specified folder. Each email is displayed as a row containing
/// the sender, subject, preview text, and timestamp. The function handles proper
/// styling and layout for each email row.
///
/// # Arguments
///
/// * `listbox` - The ListBox widget to repopulate with email rows
/// * `folder_name` - The name of the folder to retrieve emails from
/// * `emails` - HashMap containing all email data organized by folder name
///
/// # Examples
///
/// ```rust
/// use gtk4::{ListBox, prelude::*};
/// use std::collections::HashMap;
///
/// let listbox = ListBox::new();
/// let mut emails = HashMap::new();
/// emails.insert("Inbox".to_string(), vec![/* email data */]);
///
/// populate_email_list(&listbox, "Inbox", &emails);
/// ```
///
/// # Note
///
/// This function is designed to work with the Email struct and assumes emails
/// are stored in a HashMap with folder names as keys.
pub fn populate_email_list(
    listbox: &ListBox,
    folder_name: &str,
    emails: &HashMap<String, Vec<ParsedEmail>>,
) {
    // Clear existing rows
    while let Some(child) = listbox.first_child() {
        listbox.remove(&child);
    }

    // TODO: implement a method to limit number of emails shown for performance

    // Get the emails for the selected folder and create new rows
    if let Some(email_list) = emails.get(folder_name) {
        for (i, e) in email_list.iter().enumerate() {
            log::debug!("{:?}", e);
            let email_row = Box::new(Orientation::Horizontal, 0);
            email_row.set_margin_start(8);
            email_row.set_margin_end(8);
            email_row.set_margin_top(4);
            email_row.set_margin_bottom(4);
            email_row.add_css_class("email-row");

            // TODO: Replace with actual selection logic
            // Hardcoded selection for demonstration
            if i == 1 {
                email_row.add_css_class("selected");
            }

            let content_box = Box::new(Orientation::Vertical, 2);
            content_box.set_hexpand(true);

            let sender_label = Label::new(e.from.as_deref());
            sender_label.set_halign(gtk4::Align::Start);
            sender_label.add_css_class("email-sender");
            sender_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
            content_box.append(&sender_label);

            let subject_label = Label::new(e.subject.as_deref());
            subject_label.set_halign(gtk4::Align::Start);
            subject_label.add_css_class("email-subject");
            subject_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);

            let preview_label = Label::new(Some(&e.preview()));
            preview_label.set_halign(gtk4::Align::Start);
            preview_label.add_css_class("email-preview");
            preview_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);

            content_box.append(&subject_label);
            content_box.append(&preview_label);

            let time_label = Label::new(Some(&e.time_string()));
            time_label.set_halign(gtk4::Align::End);
            time_label.set_valign(gtk4::Align::Start);
            time_label.add_css_class("email-time");

            email_row.append(&content_box);
            email_row.append(&time_label);

            listbox.append(&email_row);
        }
    }
}

// FIXME: deprecate this when IMAP is implemented
/// Generates sample email data for testing and demonstration purposes.
///
/// This function creates a HashMap of email data organized by folder names,
/// with each folder containing a collection of randomly generated emails.
/// It uses the `fake` crate to generate realistic-looking email subjects,
/// sender addresses, and body content for development and testing.
///
/// # Returns
///
/// A HashMap where:
/// * Keys are folder names (e.g., "📥 Inbox", "📤 Sent")
/// * Values are vectors of Email structs with generated content
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
///
/// let emails = generate_sample_emails();
/// let inbox_emails = emails.get("📥 Inbox").unwrap();
/// assert!(inbox_emails.len() > 0);
/// ```
///
/// # Note
///
/// This is a temporary function that will be deprecated when IMAP
/// integration is implemented for loading real email data.
pub fn generate_sample_emails() -> HashMap<String, Vec<ParsedEmail>> {
    let mut emails: HashMap<String, Vec<ParsedEmail>> = HashMap::new();
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
            let time = format!(
                "{}:{:02} {}",
                (1..12).fake::<u8>(),
                (0..59).fake::<u8>(),
                if Faker.fake::<bool>() { "AM" } else { "PM" }
            );

            emails
                .entry(f.to_string())
                .or_insert_with(Vec::new)
                .push(ParsedEmail {
                    subject: Some(subject),
                    from: Some(sender),
                    to: None,
                    body_text: Some(body.clone()),
                    body_html: None,
                    timestamp: Some(time),
                });
        }
    }
    emails
}

/// Populates the email viewer pane with the selected email's content.
///
/// This function updates the email viewer interface to display the selected
/// email's information. It clears the previous content and populates the
/// header section with sender, subject, and timestamp information, then
/// loads the email body into the WebKit WebView for display.
///
/// # Arguments
///
/// * `email_header_rc` - Shared reference to the email header container Box
/// * `webkit_box_rc` - Shared reference to the WebKit WebView for content display
/// * `selected_email` - Reference to the Email struct to display
///
/// # Examples
///
/// ```rust
/// use gtk4::{Box, Label, prelude::*};
/// use webkit6::WebView;
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// let email = Email::new(
///     "Test Subject".to_string(),
///     "sender@example.com".to_string(),
///     "Preview text...".to_string(),
///     "3:45 PM".to_string(),
///     "<h1>Email Content</h1>".to_string(),
/// );
///
/// let header_box = Rc::new(RefCell::new(Box::new(gtk4::Orientation::Vertical, 0)));
/// let webview = Rc::new(RefCell::new(WebView::new()));
///
/// populate_email_viewer(header_box, webview, &email);
/// ```
fn populate_email_viewer(
    email_header_rc: Rc<RefCell<Box>>,
    webkit_box_rc: Rc<RefCell<WebView>>,
    selected_email: &ParsedEmail,
) {
    log::info!(
        "Populating email viewer for email: {:?}",
        selected_email.subject
    );
    let subject = Label::new(selected_email.subject.as_deref());
    subject.set_halign(gtk4::Align::Start);
    subject.add_css_class("viewer-subject");

    log::info!("Selected email from: {:?}", selected_email.from.as_deref());
    let from = Label::new(Some(
        selected_email.from.as_deref().unwrap_or("Unknown Sender"),
    ));
    from.set_halign(gtk4::Align::Start);
    from.add_css_class("viewer-header");

    log::info!(
        "Selected email time: {:?}",
        selected_email.timestamp.as_deref()
    );
    let time = Label::new(Some(
        selected_email
            .timestamp
            .as_deref()
            .unwrap_or("Unknown Time"),
    ));
    from.set_halign(gtk4::Align::Start);
    from.add_css_class("viewer-header");

    while let Some(child) = email_header_rc.borrow().first_child() {
        email_header_rc.borrow().remove(&child);
    }
    email_header_rc.borrow_mut().append(&from);
    email_header_rc.borrow_mut().append(&subject);
    email_header_rc.borrow_mut().append(&time);

    //TODO: Find a better way to handle HTML vs Plain Text emails
    log::info!("Loading email body into WebView");
    webkit_box_rc
        .borrow()
        .load_html(&selected_email.body_text.as_deref().unwrap(), None);
}
