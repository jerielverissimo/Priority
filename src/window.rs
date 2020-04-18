use gtk::prelude::*;

use std::collections::HashMap;

use crate::widgets::{window_headerbar::WindowHeaderbar, InnerWidget};

pub struct Window {
    pub widget: gtk::ApplicationWindow,
    pub widgets: HashMap<&'static str, Box<dyn InnerWidget>>,
    builder: gtk::Builder,
}

impl Window {
    pub fn new() -> Self {
        let glade_src = include_str!("../data/resources/ui/window.ui");
        let builder = gtk::Builder::new_from_string(glade_src);
        get_widget!(builder, gtk::ApplicationWindow, window);

        let widgets = HashMap::new();

        let mut window_widget = Window {
            widget: window,
            widgets,
            builder,
        };

        window_widget.load_widgets();
        window_widget
    }

    fn load_widgets(&mut self) {
        self.widgets.insert(
            "window_headerbar",
            Box::new(WindowHeaderbar::new(&self.builder)),
        );
    }
}
