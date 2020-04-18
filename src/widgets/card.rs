use gtk::prelude::*;

use super::InnerWidget;

pub struct Card {
    pub row: gtk::ListBoxRow,
    pub title: String,
    pub description: Option<String>,
}

impl Card {
    pub fn new(title: String) -> Self {
        let row = gtk::ListBoxRow::new();
        let description = None;

        Self::setup_card(&row, &title, &description);

        Card {
            row,
            title,
            description,
        }
    }

    fn setup_card(row: &gtk::ListBoxRow, title: &str, description: &Option<String>) {
        row.set_widget_name("card"); // add css class card

        let card_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let title = gtk::Label::new(Some(title));

        card_box.add(&title);

        row.add(&card_box);
        row.show_all();
    }
}

impl InnerWidget for Card {}
