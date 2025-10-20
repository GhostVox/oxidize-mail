use gtk4::prelude::*;
use gtk4::{glib, ApplicationWindow, Box, CheckButton, Label, Orientation, Window};
use std::cell::RefCell;
use std::rc::Rc;

use crate::config::{AppConfig, ColorScheme};

/// Creates and displays a settings window for the application.
///
/// # Arguments
///
/// * `parent` - Parent window for the dialog
/// * `config` - Shared reference to application configuration
///
/// # Examples
///
/// ```
/// show_settings_dialog(&window, config_rc.clone());
/// ```
pub fn show_settings_dialog(parent: &ApplicationWindow, config: Rc<RefCell<AppConfig>>) -> Window {
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

/// Creates the appearance settings section with theme toggles.
///
/// # Arguments
///
/// * `config` - Shared reference to application configuration
/// * `parent` - Parent window to apply theme changes to
///
/// # Returns
///
/// A Box containing the appearance settings widgets
fn create_appearance_section(config: Rc<RefCell<AppConfig>>, parent: &ApplicationWindow) -> Box {
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

/// Applies the configured theme to the application window.
///
/// # Arguments
///
/// * `window` - Application window to apply theme to
/// * `config` - Application configuration containing theme preference
fn apply_theme_to_window(window: &ApplicationWindow, config: &Rc<RefCell<AppConfig>>) {
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

