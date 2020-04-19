use gtk::prelude::*;

use super::InnerWidget;

#[derive(Clone)]
pub struct Card {
    pub row: gtk::ListBoxRow,
    pub title: String,
    pub description: Option<String>,
    img: Option<gtk::Image>,
}

impl Card {
    pub fn new(title: String) -> Self {
        let row = gtk::ListBoxRow::new();
        row.get_style_context().add_class("card");
        let description = Some(String::from("description"));

        let img = gtk::Image::new_from_icon_name(Some("application-images"), gtk::IconSize::Button);
        img.set_pixel_size(100);

        Self::setup_card(&row, &title, &description, &img);

        row.show_all();

        Card {
            row,
            title,
            description,
            img: Some(img),
        }
    }

    fn setup_card(
        row: &gtk::ListBoxRow,
        title: &str,
        description: &Option<String>,
        img: &gtk::Image,
    ) {
        row.set_widget_name("card"); // add css class card

        let card_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let title = gtk::Label::new(Some(title));
        let description: &str = &description.as_ref().unwrap();
        let description = gtk::Label::new(Some(description));

        card_box.add(img);
        card_box.add(&title);
        card_box.add(&description);

        row.add(&card_box);
    }
}

impl InnerWidget for Card {}
