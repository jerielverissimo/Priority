use gio::prelude::*;
use gtk::prelude::*;

use std::env;

use crate::config;

pub struct Application {
    app: gtk::Application,
}

impl Application {
    pub fn new() -> Self {
        let app = gtk::Application::new(Some(config::APP_ID), gio::ApplicationFlags::FLAGS_NONE)
            .expect("Application::new failed");

        let application = Self { app };

        application.setup_signals();
        application.setup_gactions();

        application
    }

    fn setup_gactions(&self) {
        // Quit
        action!(
            self.app,
            "quit",
            clone!(@strong self.app as app => move |_, _| {
                app.quit();
            })
        );
        self.app.set_accels_for_action("app.quit", &["<primary>q"]);
    }

    fn setup_signals(&self) {
        self.app.connect_activate(|app| {
            let win = gtk::ApplicationWindow::new(app);

            win.set_default_size(320, 240);
            win.set_title("Priority");

            win.show_all();
        });
    }

    pub fn run(&self) {
        let args: Vec<String> = env::args().collect();
        self.app.run(&args);
    }
}
