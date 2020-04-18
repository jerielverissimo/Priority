use gio::prelude::*;
use gtk::prelude::*;

use std::env;

pub(crate) struct Application {
    app: gtk::Application,
}

impl Application {
    pub(crate) fn new() -> Self {
        let app = gtk::Application::new(
            Some("com.github.jerielverissimo.priority"),
            gio::ApplicationFlags::FLAGS_NONE,
        )
        .expect("Application::new failed");

        let application = Self { app };

        application.setup_signals();

        application
    }

    fn setup_signals(&self) {
        self.app.connect_activate(|app| {
            let win = gtk::ApplicationWindow::new(app);

            win.set_default_size(320, 240);
            win.set_title("Roots");

            win.show_all();
        });
    }

    pub fn run(&self) {
        let args: Vec<String> = env::args().collect();
        self.app.run(&args);
    }
}
