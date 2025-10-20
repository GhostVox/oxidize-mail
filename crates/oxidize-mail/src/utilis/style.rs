use crate::config;
use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, CssProvider, Settings};
use std::cell::RefCell;
use std::rc::Rc;
/// Loads the CSS from resources and applies it to the application display.
///
/// # Examples
///
/// ```
/// load_css();
/// ```
pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/com/oxidize/mail/css/style.css");
    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
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
pub fn apply_theme(window: &ApplicationWindow, config: Rc<RefCell<config::AppConfig>>) {
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
