use glib;
use gtk4::gio;
/// Register the GResources for the application.
///
/// # Examples
///
/// ```
/// register_resources();
/// ```
pub fn register_resources() {
    let resource_bytes = glib::Bytes::from_static(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/oxidize-mail.gresource"
    )));
    let resource = gio::Resource::from_data(&resource_bytes).expect("Failed to load GResource");
    gio::resources_register(&resource);
}
