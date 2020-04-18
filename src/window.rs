use gtk::prelude::*;

use crate::config::APP_ID;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
    builder: gtk::Builder,
}

impl Window {
    pub fn new() -> Self {
        let glade_src = include_str!("../data/resources/ui/window.ui");
        let builder = gtk::Builder::new_from_string(glade_src);
        get_widget!(builder, gtk::ApplicationWindow, window);

        let window_widget = Window {
            widget: window,
            builder,
        };

        //window_widget.load_widgets();
        //window_widget.init();
        window_widget
    }

    fn load_widgets(&mut self) {
        unimplemented!();
    }

    fn init(&self) {
        unimplemented!();
    }
}
