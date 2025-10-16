use fake::faker::internet::en::SafeEmail;
use fake::faker::lorem::en::{Sentence, Words};
use fake::Fake;
use fake::Faker;
// use fake::faker::name::en::Name;
use gtk4::glib::clone;
use gtk4::prelude::*;
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

fn register_resources() {
    let resource_bytes = glib::Bytes::from_static(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/oxidize-mail.gresource"
    )));
    let resource = gio::Resource::from_data(&resource_bytes).expect("Failed to load GResource");
    gio::resources_register(&resource);
}

fn build_ui(app: &Application) {
    load_css();
    let settings_rc = Rc::new(RefCell::new(config::AppConfig::load()));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Oxidize Mail")
        .default_width(1400)
        .default_height(900)
        .build();

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

    // --- Refactoring Start ---

    // 1. Create the email list widgets, getting a reference to the ListBox
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
    let email_viewer = create_email_viewer();
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

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/com/oxidize/mail/css/style.css");
    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/**
* Creates the folder sidebar.
* Now accepts a reference to the email ListBox to update it directly.
*/
fn create_folder_sidebar(
    title_label: Rc<RefCell<Label>>,
    settings: Rc<RefCell<config::AppConfig>>,
    emails: Rc<RefCell<HashMap<String, Vec<(String, String, String, String)>>>>,
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
    sidebar
}

/**
* Creates the widgets for the email list pane.
* Returns the main container and the ListBox inside it.
*/
fn create_email_list_widgets() -> (Box, ListBox) {
    let list_container = Box::new(Orientation::Vertical, 0);
    list_container.set_vexpand(true);
    list_container.set_hexpand(true);
    list_container.add_css_class("email-list");

    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(PolicyType::Never, PolicyType::Automatic);
    scrolled.set_vexpand(true);

    let listbox = ListBox::new();

    scrolled.set_child(Some(&listbox));
    list_container.append(&scrolled);
    (list_container, listbox)
}

/**
* Clears and repopulates the email ListBox with new data.
*/
fn populate_email_list(
    listbox: &ListBox,
    folder_name: &str,
    emails: &HashMap<String, Vec<(String, String, String, String)>>,
) {
    // Clear existing rows
    while let Some(child) = listbox.first_child() {
        listbox.remove(&child);
    }

    // Get the emails for the selected folder and create new rows
    if let Some(email_list) = emails.get(folder_name) {
        for (i, (subject, sender, preview, time)) in email_list.iter().enumerate() {
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

            if !sender.is_empty() {
                let sender_label = Label::new(Some(sender));
                sender_label.set_halign(gtk4::Align::Start);
                sender_label.add_css_class("email-sender");
                sender_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
                content_box.append(&sender_label);
            }

            let subject_label = Label::new(Some(subject));
            subject_label.set_halign(gtk4::Align::Start);
            subject_label.add_css_class("email-subject");
            subject_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);

            let preview_label = Label::new(Some(preview));
            preview_label.set_halign(gtk4::Align::Start);
            preview_label.add_css_class("email-preview");
            preview_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);

            content_box.append(&subject_label);
            content_box.append(&preview_label);

            let time_label = Label::new(Some(time));
            time_label.set_halign(gtk4::Align::End);
            time_label.set_valign(gtk4::Align::Start);
            time_label.add_css_class("email-time");

            email_row.append(&content_box);
            email_row.append(&time_label);

            listbox.append(&email_row);
        }
    }
}

// --- Functions below this line are unchanged ---

fn generate_sample_emails() -> HashMap<String, Vec<(String, String, String, String)>> {
    let mut emails: HashMap<String, Vec<(String, String, String, String)>> = HashMap::new();
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
            let preview: String = Words(8..15).fake::<Vec<String>>().join(" ") + "...";
            let time = format!(
                "{}:{:02} {}",
                (1..12).fake::<u8>(),
                (0..59).fake::<u8>(),
                if Faker.fake::<bool>() { "AM" } else { "PM" }
            );

            emails
                .entry(f.to_string())
                .or_insert_with(Vec::new)
                .push((subject, sender, preview, time));
        }
    }
    emails
}

fn create_email_viewer() -> Box {
    // This function remains the same as it's just a static display for now.
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

    let subject = Label::new(Some("My Best Buy"));
    subject.set_halign(gtk4::Align::Start);
    subject.add_css_class("viewer-subject");

    let from = Label::new(Some("From: bret637@gmail.com"));
    from.set_halign(gtk4::Align::Start);
    from.add_css_class("viewer-header");

    let reply_to = Label::new(Some("Reply-To: Best Buy"));
    reply_to.set_halign(gtk4::Align::Start);
    reply_to.add_css_class("viewer-header");

    header_box.append(&subject);
    header_box.append(&from);
    header_box.append(&reply_to);

    content.append(&header_box);
    // ... rest of the hardcoded viewer content is the same ...

    // Best Buy promotional content
    let promo_box = Box::new(Orientation::Vertical, 0);

    // Blue header with celebration message
    let blue_header = Box::new(Orientation::Vertical, 12);
    blue_header.add_css_class("bestbuy-header");
    blue_header.set_margin_top(20);

    let celebrate = Label::new(Some("It's time to celebrate, Brent"));
    celebrate.add_css_class("celebrate-text");

    let amount = Label::new(Some("$5"));
    amount.add_css_class("certificate-amount");

    let cert_label = Label::new(Some("Certificate*"));
    cert_label.add_css_class("certificate-label");

    blue_header.append(&celebrate);
    blue_header.append(&amount);
    blue_header.append(&cert_label);

    // Reward description
    let description_box = Box::new(Orientation::Vertical, 12);
    description_box.add_css_class("reward-description");
    description_box.set_margin_start(20);
    description_box.set_margin_end(20);
    description_box.set_margin_top(20);
    description_box.set_margin_bottom(20);

    let desc_text = Label::new(Some("Let's make today even more special. Here's a $5 monthly reward to use toward your next Best Buy® purchase. Enjoy the savings and keep enjoying your membership benefits."));
    desc_text.set_wrap(true);
    desc_text.set_justify(gtk4::Justification::Center);

    let shop_btn = Label::new(Some("Shop Now"));
    shop_btn.add_css_class("shop-button");
    shop_btn.set_margin_top(12);

    let member_id = Label::new(Some("Member ID: 4127480758"));
    member_id.set_margin_top(20);
    member_id.add_css_class("caption");

    let cert_number = Label::new(Some("Certificate Number: 3253413911"));
    cert_number.add_css_class("caption");

    let exp_date = Label::new(Some("Expiration Date: 9/30/25"));
    exp_date.add_css_class("caption");

    description_box.append(&desc_text);
    description_box.append(&shop_btn);
    description_box.append(&member_id);
    description_box.append(&cert_number);
    description_box.append(&exp_date);

    promo_box.append(&blue_header);
    promo_box.append(&description_box);

    content.append(&header_box);
    content.append(&promo_box);
    scrolled.set_child(Some(&content));
    viewer.append(&scrolled);
    viewer
}
