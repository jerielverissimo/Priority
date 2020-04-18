use gtk::prelude::*;

use crate::config::APP_ID;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
    builder: gtk::Builder,
    settings: gio::Settings,
}

impl Window {
    pub fn new() -> Self {
        let settings = gio::Settings::new(APP_ID);

        let builder = gtk::Builder::new();
        get_widget!(builder, gtk::ApplicationWindow, window);

        let mut window_widget = Window {
            widget: window,
            builder,
            settings,
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
