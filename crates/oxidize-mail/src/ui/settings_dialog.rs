use gtk4::prelude::*;
use gtk4::{glib, ApplicationWindow, Box, CheckButton, Label, Orientation, Window};
use oxidize_mail_types::{ColorScheme, UserConfig};
use std::cell::RefCell;
use std::rc::Rc;

/// Creates and displays a modal settings window for the application.
///
/// This function constructs a settings dialog window with appearance preferences
/// and other configuration options. The window is modal and transient to the
/// parent window, ensuring proper focus management. It includes theme selection
/// controls and automatically saves configuration changes when closed.
///
/// # Arguments
///
/// * `parent` - The parent ApplicationWindow that will own this modal dialog
/// * `config` - Shared reference to the UserConfig for reading and updating settings
///
/// # Returns
///
/// The created Window instance for the settings dialog
///
/// # Examples
///
/// ```rust
/// use gtk4::{ApplicationWindow, prelude::*};
/// use oxidize_mail_types::UserConfig;
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// let parent_window = ApplicationWindow::new();
/// let config = Rc::new(RefCell::new(UserConfig::default()));
/// let settings_window = show_settings_dialog(&parent_window, config);
/// ```
pub fn show_settings_dialog(parent: &ApplicationWindow, config: Rc<RefCell<UserConfig>>) -> Window {
    let window = Window::builder()
        .title("Settings")
        .modal(true)
        .transient_for(parent)
        .default_width(400)
        .default_height(300)
        .build();

    // Create content area
    let content_box = Box::new(Orientation::Vertical, 12);
    content_box.set_margin_start(20);
    content_box.set_margin_end(20);
    content_box.set_margin_top(20);
    content_box.set_margin_bottom(20);
    content_box.set_vexpand(true);

    // Appearance Section
    let appearance_section = create_appearance_section(config.clone(), parent);
    content_box.append(&appearance_section);

    // Close button at the bottom
    let button_box = Box::new(Orientation::Horizontal, 0);
    button_box.set_halign(gtk4::Align::End);
    button_box.set_margin_top(12);

    let close_button = gtk4::Button::with_label("Close");
    close_button.add_css_class("suggested-action");

    let window_weak = window.downgrade();
    close_button.connect_clicked(glib::clone!(
        #[strong]
        config,
        move |_| {
            // Save config when closing
            config.borrow().save();
            if let Some(win) = window_weak.upgrade() {
                win.close();
            }
        }
    ));

    button_box.append(&close_button);
    content_box.append(&button_box);

    window.set_child(Some(&content_box));
    window.present();
    window
}

/// Creates the appearance settings section with color scheme selection controls.
///
/// This function constructs a settings section containing radio buttons for
/// theme selection (Light, Dark, System). The radio buttons are grouped together
/// and their state is synchronized with the current configuration. When toggled,
/// the theme is immediately applied to the parent window and the configuration
/// is updated accordingly.
///
/// # Arguments
///
/// * `config` - Shared reference to the UserConfig for reading and updating theme preferences
/// * `parent` - The parent ApplicationWindow to apply theme changes to immediately
///
/// # Returns
///
/// A Box widget containing the complete appearance settings section with header and controls
///
/// # Examples
///
/// ```rust
/// use gtk4::{Box, ApplicationWindow, prelude::*};
/// use oxidize_mail_types::UserConfig;
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// let parent_window = ApplicationWindow::new();
/// let config = Rc::new(RefCell::new(UserConfig::default()));
/// let appearance_section = create_appearance_section(config, &parent_window);
/// ```
fn create_appearance_section(config: Rc<RefCell<UserConfig>>, parent: &ApplicationWindow) -> Box {
    let section = Box::new(Orientation::Vertical, 8);

    // Section header
    let header = Label::new(Some("Appearance"));
    header.set_halign(gtk4::Align::Start);
    header.add_css_class("title-3");
    section.append(&header);

    // Theme selection label
    let theme_label = Label::new(Some("Color Scheme:"));
    theme_label.set_halign(gtk4::Align::Start);
    theme_label.add_css_class("heading");
    section.append(&theme_label);

    // Radio buttons for theme selection
    let light_radio = CheckButton::with_label("Light");
    let dark_radio = CheckButton::with_label("Dark");
    let system_radio = CheckButton::with_label("System");

    // Set radio button group
    dark_radio.set_group(Some(&light_radio));
    system_radio.set_group(Some(&light_radio));

    // Set initial state based on config
    match config.borrow().get_preferred_color_scheme() {
        ColorScheme::Light => light_radio.set_active(true),
        ColorScheme::Dark => dark_radio.set_active(true),
        ColorScheme::System => system_radio.set_active(true),
    }

    // Create a container for radio buttons with proper spacing
    let radio_box = Box::new(Orientation::Vertical, 4);
    radio_box.set_margin_start(12);
    radio_box.append(&light_radio);
    radio_box.append(&dark_radio);
    radio_box.append(&system_radio);

    section.append(&radio_box);

    // Connect signals for theme changes
    // We need to clone parent_weak for each closure
    let parent_weak_1 = parent.downgrade();
    let parent_weak_2 = parent.downgrade();
    let parent_weak_3 = parent.downgrade();

    light_radio.connect_toggled(glib::clone!(
        #[strong]
        config,
        move |button| {
            if button.is_active() {
                config.borrow_mut().update_color_scheme(ColorScheme::Light);
                if let Some(window) = parent_weak_1.upgrade() {
                    apply_theme_to_window(&window, &config);
                }
            }
        }
    ));

    dark_radio.connect_toggled(glib::clone!(
        #[strong]
        config,
        move |button| {
            if button.is_active() {
                config.borrow_mut().update_color_scheme(ColorScheme::Dark);
                if let Some(window) = parent_weak_2.upgrade() {
                    apply_theme_to_window(&window, &config);
                }
            }
        }
    ));

    system_radio.connect_toggled(glib::clone!(
        #[strong]
        config,
        move |button| {
            if button.is_active() {
                config.borrow_mut().update_color_scheme(ColorScheme::System);
                if let Some(window) = parent_weak_3.upgrade() {
                    apply_theme_to_window(&window, &config);
                }
            }
        }
    ));

    section
}

/// Applies the configured color scheme theme to the application window.
///
/// This function reads the user's preferred color scheme from the configuration
/// and applies the appropriate theme to the window. For the System theme, it
/// checks GTK's system preference. The function adds or removes the "dark" CSS
/// class from the window to control theming.
///
/// # Arguments
///
/// * `window` - The ApplicationWindow to apply the theme styling to
/// * `config` - Shared reference to UserConfig containing the theme preference
///
/// # Examples
///
/// ```rust
/// use gtk4::{ApplicationWindow, prelude::*};
/// use oxidize_mail_types::UserConfig;
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// let window = ApplicationWindow::new();
/// let config = Rc::new(RefCell::new(UserConfig::default()));
/// apply_theme_to_window(&window, &config);
/// ```
fn apply_theme_to_window(window: &ApplicationWindow, config: &Rc<RefCell<UserConfig>>) {
    let should_use_dark = match config.borrow().get_preferred_color_scheme() {
        ColorScheme::Light => false,
        ColorScheme::Dark => true,
        ColorScheme::System => {
            // Check GTK system preference
            if let Some(settings) = gtk4::Settings::default() {
                settings.property::<bool>("gtk-application-prefer-dark-theme")
            } else {
                false
            }
        }
    };

    // Add or remove the "dark" CSS class
    if should_use_dark {
        window.add_css_class("dark");
    } else {
        window.remove_css_class("dark");
    }
}
