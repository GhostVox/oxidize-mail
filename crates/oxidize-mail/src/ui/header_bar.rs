use gtk4::prelude::*;
use gtk4::{self, HeaderBar, Label};
use oxidize_mail_types::UserConfig;
use std::cell::RefCell;
use std::rc::Rc;

/// Creates the application header bar with title and navigation controls.
///
/// This function constructs the main application header bar that displays the
/// current folder name and provides standard window controls. The header bar
/// includes a dynamically updated title label that reflects the currently
/// selected email folder from the user's configuration.
///
/// # Arguments
///
/// * `settings_rc` - Shared reference to UserConfig for accessing the selected folder
///
/// # Returns
///
/// A tuple containing:
/// * `HeaderBar` - The configured header bar widget ready for window attachment
/// * `Rc<RefCell<Label>>` - Shared reference to the title label for dynamic updates
///
/// # Examples
///
/// ```rust
/// use gtk4::{HeaderBar, Label, prelude::*};
/// use oxidize_mail_types::UserConfig;
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// let settings = Rc::new(RefCell::new(UserConfig::default()));
/// let (header_bar, title_label) = create_headerbar(settings);
///
/// // The header can now be set on a window
/// // window.set_titlebar(Some(&header_bar));
///
/// // The title can be updated dynamically
/// title_label.borrow().set_text("New Folder");
/// ```
pub fn create_headerbar(settings_rc: Rc<RefCell<UserConfig>>) -> (HeaderBar, Rc<RefCell<Label>>) {
    let header = HeaderBar::new();
    header.set_show_title_buttons(true);
    let title = Label::new(Some(&settings_rc.borrow().get_selected_folder()));
    title.add_css_class("title");
    let title_rc = Rc::new(RefCell::new(title.clone()));
    header.set_title_widget(Some(&title));

    (header, title_rc)
}
