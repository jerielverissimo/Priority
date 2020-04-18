use gio::prelude::*;
use gtk::prelude::*;

use std::env;

use crate::config;
use crate::window::Window;

pub struct Application {
    app: gtk::Application,
    window: Window,
}

impl Application {
    pub fn new() -> Self {
        let app = gtk::Application::new(Some(config::APP_ID), gio::ApplicationFlags::FLAGS_NONE)
            .expect("Application::new failed");
        let window = Window::new();

        let application = Self { app, window };

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
        self.app
            .connect_activate(clone!(@weak self.window.widget as window => move |app| {
                window.set_application(Some(app));
                app.add_window(&window);
                window.present();
            }));
    }

    pub fn run(&self) {
        let args: Vec<String> = env::args().collect();
        self.app.run(&args);
    }
}
