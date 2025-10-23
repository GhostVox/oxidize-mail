use glib;
use gtk4::gio;

/// Registers the compiled GResource bundle with the GIO resource system.
///
/// This function loads the embedded GResource file (compiled from the application's
/// resource files) and registers it with GIO's global resource registry. This makes
/// all bundled resources (CSS files, UI definitions, icons, etc.) available to the
/// application at runtime through the GResource API.
///
/// The GResource file is embedded at compile time using `include_bytes!` and the
/// build system's `OUT_DIR` environment variable to locate the compiled resource bundle.
///
/// # Examples
///
/// ```rust
/// use glib;
/// use gtk4::gio;
///
/// // Call this early in your application startup, before creating any UI
/// register_resources();
///
/// // Now you can access resources like:
/// // gio::resources_lookup_data("/com/oxidize/mail/css/style.css", gio::ResourceLookupFlags::NONE)
/// ```
///
/// # Panics
///
/// This function will panic if the embedded GResource data is invalid or corrupted,
/// or if the resource registration fails for any reason.
pub fn register_resources() {
    let resource_bytes = glib::Bytes::from_static(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/oxidize-mail.gresource"
    )));
    let resource = gio::Resource::from_data(&resource_bytes).expect("Failed to load GResource");
    gio::resources_register(&resource);
}
