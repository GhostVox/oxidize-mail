use gtk4::prelude::*;
use gtk4::{self, HeaderBar, Label};
use std::cell::RefCell;
use std::rc::Rc;
use oxidize_mail_types::UserConfig;

// Header bar setup

pub fn create_headerbar(
    settings_rc: Rc<RefCell<UserConfig>>,
) -> (HeaderBar, Rc<RefCell<Label>>) {
    let header = HeaderBar::new();
    header.set_show_title_buttons(true);
    let title = Label::new(Some(&settings_rc.borrow().get_selected_folder()));
    title.add_css_class("title");
    let title_rc = Rc::new(RefCell::new(title.clone()));
    header.set_title_widget(Some(&title));

    (header, title_rc)
}
