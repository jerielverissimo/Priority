use gtk::prelude::*;

use super::InnerWidget;

pub struct WindowHeaderbar {
    pub widget: gtk::HeaderBar,
    pub home_button: gtk::Button,
}

impl WindowHeaderbar {
    pub fn new(builder: &gtk::Builder) -> Self {
        get_widget!(builder, gtk::HeaderBar, window_headerbar);
        get_widget!(builder, gtk::Button, home_button);

        Self {
            widget: window_headerbar,
            home_button,
        }
    }
}

impl InnerWidget for WindowHeaderbar {}
