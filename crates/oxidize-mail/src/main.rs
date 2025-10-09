use gtk4::prelude::*;
use gtk4::{
    glib, Application, ApplicationWindow, Box, HeaderBar, Label, ListBox, Orientation, Paned,
    PolicyType, ScrolledWindow,
};

const APP_ID: &str = "com.oxidize.mail";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Oxidize Mail")
        .default_width(1200)
        .default_height(800)
        .build();

    // Header bar
    let header = HeaderBar::new();
    header.set_show_title_buttons(true);
    window.set_titlebar(Some(&header));

    // Main container - horizontal paned (3 columns)
    let main_paned = Paned::new(Orientation::Horizontal);

    // LEFT PANE: Folder sidebar
    let folder_sidebar = create_folder_sidebar();
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
    main_paned.set_position(200); // Sidebar width
    content_paned.set_position(400); // Email list width

    window.set_child(Some(&main_paned));
    window.present();
}

fn create_folder_sidebar() -> Box {
    let sidebar = Box::new(Orientation::Vertical, 0);
    sidebar.set_width_request(200);
    sidebar.set_vexpand(true); // ← IMPORTANT
    sidebar.set_hexpand(true); // ← IMPORTANT

    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(PolicyType::Never, PolicyType::Automatic); // ← IMPORTANT
    scrolled.set_vexpand(true); // ← IMPORTANT
    scrolled.set_hexpand(true); // ← IMPORTANT

    let listbox = ListBox::builder()
        .css_classes(vec!["navigation-sidebar"])
        .build();

    // Add folders
    let folders = vec!["📥 Inbox", "📤 Sent", "✏️ Drafts", "🗑️ Trash", "⭐ Starred"];
    for folder in folders {
        let label = Label::new(Some(folder));
        label.set_halign(gtk4::Align::Start);
        label.set_margin_start(12);
        label.set_margin_end(12);
        label.set_margin_top(8);
        label.set_margin_bottom(8);
        listbox.append(&label);
    }

    scrolled.set_child(Some(&listbox));
    sidebar.append(&scrolled);
    sidebar
}

fn create_email_list() -> Box {
    let list_box = Box::new(Orientation::Vertical, 0);
    list_box.set_vexpand(true); // ← IMPORTANT
    list_box.set_hexpand(true); // ← IMPORTANT

    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(PolicyType::Never, PolicyType::Automatic); // ← IMPORTANT
    scrolled.set_vexpand(true); // ← IMPORTANT
    scrolled.set_hexpand(true); // ← IMPORTANT

    let listbox = ListBox::new();

    // Sample emails
    for i in 1..=20 {
        let email_row = Box::new(Orientation::Vertical, 4);
        email_row.set_margin_start(12);
        email_row.set_margin_end(12);
        email_row.set_margin_top(8);
        email_row.set_margin_bottom(8);

        let subject = Label::new(Some(&format!("Email Subject {}", i)));
        subject.set_halign(gtk4::Align::Start);
        subject.add_css_class("heading");

        let sender = Label::new(Some(&format!("sender{}@example.com", i)));
        sender.set_halign(gtk4::Align::Start);
        sender.add_css_class("dim-label");

        let preview = Label::new(Some("This is a preview of the email content..."));
        preview.set_halign(gtk4::Align::Start);
        preview.add_css_class("caption");

        email_row.append(&subject);
        email_row.append(&sender);
        email_row.append(&preview);

        listbox.append(&email_row);
    }

    scrolled.set_child(Some(&listbox));
    list_box.append(&scrolled);
    list_box
}

fn create_email_viewer() -> Box {
    let viewer = Box::new(Orientation::Vertical, 12);
    viewer.set_vexpand(true); // ← IMPORTANT
    viewer.set_hexpand(true); // ← IMPORTANT
    viewer.set_margin_start(20);
    viewer.set_margin_end(20);
    viewer.set_margin_top(20);
    viewer.set_margin_bottom(20);

    // Email header
    let subject = Label::new(Some("Email Subject"));
    subject.set_halign(gtk4::Align::Start);
    subject.add_css_class("title-1");

    let from = Label::new(Some("From: sender@example.com"));
    from.set_halign(gtk4::Align::Start);

    let date = Label::new(Some("Date: October 8, 2025"));
    date.set_halign(gtk4::Align::Start);
    date.add_css_class("dim-label");

    // Email body
    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(PolicyType::Automatic, PolicyType::Automatic); // ← IMPORTANT
    scrolled.set_vexpand(true); // ← IMPORTANT
    scrolled.set_hexpand(true); // ← IMPORTANT

    let body = Label::new(Some("Email body content goes here...\n\nThis is where the full email message would be displayed.\n\nLorem ipsum dolor sit amet, consectetur adipiscing elit.\n\nSed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n\nUt enim ad minim veniam, quis nostrud exercitation ullamco laboris.\n\nNisi ut aliquip ex ea commodo consequat."));
    body.set_halign(gtk4::Align::Start);
    body.set_valign(gtk4::Align::Start);
    body.set_wrap(true);
    body.set_margin_top(20);

    scrolled.set_child(Some(&body));

    viewer.append(&subject);
    viewer.append(&from);
    viewer.append(&date);
    viewer.append(&scrolled);

    viewer
}
