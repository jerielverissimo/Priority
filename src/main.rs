#[macro_use]
extern crate glib;

#[macro_use]
mod utils;
mod application;
mod config;
mod widgets;
mod window;

use application::Application;

fn main() {
    glib::set_application_name("Primary");
    glib::set_prgname(Some("priority"));

    gtk::init().expect("Unable to start GTK3");

    let app = Application::new();
    app.run();
}
