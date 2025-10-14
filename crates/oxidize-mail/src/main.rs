use gtk4::glib::clone;
use gtk4::prelude::*;
mod config;
use gtk4::{
    gdk::Display, gio, glib, Application, ApplicationWindow, Box, CssProvider, HeaderBar, Label,
    ListBox, Orientation, Paned, PolicyType, ScrolledWindow, Settings,
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
    // Include the compiled .gresource file that was created by build.rs
    let resource_bytes = glib::Bytes::from_static(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/oxidize-mail.gresource"
    )));
    let resource = gio::Resource::from_data(&resource_bytes).expect("Failed to load GResource");
    gio::resources_register(&resource);
}

fn setup_application_icon(settings: &config::AppConfig) {
    use gtk4::IconTheme;

    let icon_theme =
        IconTheme::for_display(&Display::default().expect("Could not connect to display"));

    // Add resource path to icon theme search paths
    icon_theme.add_resource_path("/com/oxidize/mail/icons");

    // Determine which icon to use based on color scheme
    let color_scheme = settings.get_preferred_color_scheme();
    let icon_name = match color_scheme {
        config::ColorScheme::Light => "oxidize-mail-light",
        config::ColorScheme::Dark => "oxidize-mail-dark",
        config::ColorScheme::System => {
            let gtk_settings = Settings::default().expect("Could not get GTK settings");
            let prefer_dark = gtk_settings.property::<bool>("gtk-application-prefer-dark-theme");
            if prefer_dark {
                "oxidize-mail-dark"
            } else {
                "oxidize-mail-light"
            }
        }
    };

    println!("Setting icon: {}", icon_name);
}

fn build_ui(app: &Application) {
    // Load custom CSS
    load_css();
    // Load the App config
    let settings = Rc::new(RefCell::new(config::AppConfig::load()));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Oxidize Mail")
        .default_width(1400)
        .default_height(900)
        .build();

    // Set the window icon based on the preferred color scheme
    window.set_icon_name(Some("oxidize-mail"));
    // Header bar
    let header = HeaderBar::new();
    header.set_show_title_buttons(true);

    let title = Label::new(Some("All Inboxes"));
    title.add_css_class("title");

    // Wrap title in Rc<RefCell> for shared mutable access
    let title_rc = Rc::new(RefCell::new(title.clone()));

    header.set_title_widget(Some(&title));

    window.set_titlebar(Some(&header));

    // Main container - horizontal paned (3 columns)
    let main_paned = Paned::new(Orientation::Horizontal);

    // LEFT PANE: Folder sidebar
    //  clone the current title for use in the sidebar creation
    let folder_sidebar = create_folder_sidebar(title_rc.clone());
    main_paned.set_start_child(Some(&folder_sidebar));
    main_paned.set_resize_start_child(false);
    main_paned.set_shrink_start_child(false);

    // MIDDLE + RIGHT: Another paned widget
    let content_paned = Paned::new(Orientation::Horizontal);

    // MIDDLE PANE: Email list
    let email_list = create_email_list();
    content_paned.set_start_child(Some(&email_list));
    content_paned.set_resize_start_child(true);
    content_paned.set_shrink_start_child(false);

    // RIGHT PANE: Email viewer
    let email_viewer = create_email_viewer();
    content_paned.set_end_child(Some(&email_viewer));
    content_paned.set_resize_end_child(true);
    content_paned.set_shrink_end_child(false);

    main_paned.set_end_child(Some(&content_paned));
    main_paned.set_position(220); // Sidebar width
    content_paned.set_position(450); // Email list width

    window.set_child(Some(&main_paned));

    // Closure to be called when the application is about to close

    window.connect_close_request(clone!(
        #[strong]
        settings,
        move |_| {
            settings.borrow().save();
            glib::Propagation::Proceed
        }
    ));
    window.present();
}

/// This function loads the CSS from our GResource and applies it to the application.
fn load_css() {
    let provider = CssProvider::new();
    // Load CSS from the GResource using the full path
    provider.load_from_resource("/com/oxidize/mail/css/style.css");

    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/**
* Creates the folder sidebar with sections and folders.
* @param title_label A label widget to display the current folder title in the header.
* */
fn create_folder_sidebar(title_label: Rc<RefCell<Label>>) -> Box {
    let sidebar = Box::new(Orientation::Vertical, 0);
    sidebar.set_width_request(220);
    sidebar.set_vexpand(true);
    sidebar.set_hexpand(false);
    sidebar.add_css_class("navigation-sidebar");

    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(PolicyType::Never, PolicyType::Automatic);
    scrolled.set_vexpand(true);

    let listbox = ListBox::new();

    // TODO: We will need to dynamically load folders from email accounts in the future.

    // Add folder sections
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

    // Populate listbox with sections and folders
    for (section_name, folders) in sections {
        // Section header
        let header = Label::new(Some(section_name));
        header.set_halign(gtk4::Align::Start);
        header.set_margin_start(12);
        header.set_margin_top(12);
        header.set_margin_bottom(4);
        header.add_css_class("caption");
        header.add_css_class("dim-label");
        listbox.append(&header);

        // Folders
        for folder in folders {
            let label = Label::new(Some(folder));
            label.set_halign(gtk4::Align::Start);
            label.add_css_class("folder-item");
            listbox.append(&label);
        }
    }

    // Connect row-selected signal to update title
    listbox.connect_row_selected(move |_listbox, row| {
        if let Some(row) = row {
            if let Some(child) = row.child() {
                if let Ok(label) = child.downcast::<Label>() {
                    let folder_name = label.text();
                    // Only update if it's not a section header
                    if !folder_name.is_empty() && !label.has_css_class("dim-label") {
                        title_label.borrow_mut().set_text(&folder_name);
                    }
                }
            }
        }
    });

    scrolled.set_child(Some(&listbox));
    sidebar.append(&scrolled);
    sidebar
}

fn create_email_list() -> Box {
    let list_box = Box::new(Orientation::Vertical, 0);
    list_box.set_vexpand(true);
    list_box.set_hexpand(true);
    list_box.add_css_class("email-list");

    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(PolicyType::Never, PolicyType::Automatic);
    scrolled.set_vexpand(true);

    let listbox = ListBox::new();

    // TODO:: We will need to dynamically load emails from the selected folder in the future.
    //
    // Sample emails matching the screenshot
    let emails = vec![
        ("Weekend Events at Colonial Trail", "Inbox - bret637@gmail.com", "Happening now at our neighbors this weeke...", "7:47 AM"),
        ("Twitch", "Inbox - bret637@gmail.com", "YOASOBI×Yu-ki Wa WHEEI, RETURNI Your now...", "5:52 PM"),
        ("Kaiser Permanente", "Inbox - bret637@gmail.com", "Important changes to health coverage includes", "12:28 PM"),
        ("Turbotax", "Inbox - Google", "🔴 Only 3 more days to file on time!", "9:18 AM"),
        ("Niklas Gusdmar, Co-Founder", "Inbox - bret637@gmail.com", "Confessions on AI Agent Adventures", "10/7/25"),
        ("Tik Nagle Ferry Party.management company contact", "", "Success story: Why didn't I think of this?...", "9/30"),
        ("Mia Igarashi", "Inbox - Google", "Re: Proposal for reforming the way we...", "9/3/25"),
        ("My Best Buy", "Inbox - bret637@gmail.com", "Surprise, Brent! You've earned a $5 certificate. 🎊 Here $5 monthly reward, use its BYOD25, gift cards available.", "8/1/25"),
        ("Claudia Jorgenz", "Inbox - bret637@gmail.com", "483A Ocean Drive address", "8/1/25"),
    ];

    // Take each email tuple and create a row
    for (i, (subject, sender, preview, time)) in emails.iter().enumerate() {
        let email_row = Box::new(Orientation::Horizontal, 0);
        email_row.set_margin_start(8);
        email_row.set_margin_end(8);
        email_row.set_margin_top(4);
        email_row.set_margin_bottom(4);
        email_row.add_css_class("email-row");

        // Highlight "My Best Buy" email
        if i == 7 {
            email_row.add_css_class("selected");
        }

        let content_box = Box::new(Orientation::Vertical, 2);
        content_box.set_hexpand(true);

        let subject_label = Label::new(Some(subject));
        subject_label.set_halign(gtk4::Align::Start);
        subject_label.add_css_class("email-subject");
        subject_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);

        if !sender.is_empty() {
            let sender_label = Label::new(Some(sender));
            sender_label.set_halign(gtk4::Align::Start);
            sender_label.add_css_class("email-sender");
            sender_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
            content_box.append(&sender_label);
        }

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

    scrolled.set_child(Some(&listbox));
    list_box.append(&scrolled);
    list_box
}

// TODO: Replace the email viewer with a webkit view to render HTML emails.
fn create_email_viewer() -> Box {
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
