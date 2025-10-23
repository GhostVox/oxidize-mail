use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, CssProvider, Settings};
use oxidize_mail_types::{ColorScheme, UserConfig};
use std::cell::RefCell;
use std::rc::Rc;
/// Loads the CSS stylesheet from resources and applies it to the default display.
///
/// This function creates a CSS provider, loads the application's main stylesheet
/// from the GResource bundle, and applies it to the default display with application
/// priority. This ensures that the application's styling takes precedence over
/// default GTK themes but can still be overridden by user themes if needed.
///
/// The CSS file is expected to be located at `/com/oxidize/mail/css/style.css`
/// within the application's resource bundle.
///
/// # Examples
///
/// ```rust
/// use gtk4::gdk::Display;
/// use gtk4::prelude::*;
/// use gtk4::CssProvider;
///
/// // Call this early in your application startup after registering resources
/// load_css();
/// ```
///
/// # Panics
///
/// This function will panic if:
/// - No default display is available
/// - The CSS resource cannot be loaded from the bundle
/// - The CSS provider fails to parse the stylesheet
pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/com/oxidize/mail/css/style.css");
    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
/// Applies the appropriate color scheme theme to the given window based on user configuration.
///
/// This function reads the user's preferred color scheme from the configuration and
/// applies the corresponding theme to the window. It supports three modes:
/// - Light: Forces light theme by removing "dark" CSS class
/// - Dark: Forces dark theme by adding "dark" CSS class
/// - System: Uses GTK's system preference for theme selection
///
/// The theming is implemented by adding or removing the "dark" CSS class from the
/// window, which should be handled by the application's CSS stylesheet.
///
/// # Arguments
///
/// * `window` - The ApplicationWindow to apply the theme styling to
/// * `config` - Shared reference to UserConfig containing the color scheme preference
///
/// # Examples
///
/// ```rust
/// use gtk4::{ApplicationWindow, Settings, prelude::*};
/// use oxidize_mail_types::{UserConfig, ColorScheme};
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// let window = ApplicationWindow::new();
/// let config = Rc::new(RefCell::new(UserConfig::default()));
///
/// // Apply the theme based on current configuration
/// apply_theme(&window, config.clone());
///
/// // The window will now have appropriate CSS classes for theming
/// ```
///
/// # Panics
///
/// This function will panic if GTK settings cannot be retrieved when using
/// the System color scheme mode.
pub fn apply_theme(window: &ApplicationWindow, config: Rc<RefCell<UserConfig>>) {
    let should_use_dark = match config.borrow().get_preferred_color_scheme() {
        ColorScheme::Light => false,
        ColorScheme::Dark => true,
        ColorScheme::System => {
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
