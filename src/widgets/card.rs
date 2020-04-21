use gtk::prelude::*;

use super::InnerWidget;

#[derive(Clone)]
pub struct Card {
    pub row: gtk::ListBoxRow,
    pub title: String,
    pub description: Option<String>,
    img: Option<gtk::Image>,
    labels: gtk::Box,
}

impl Card {
    pub fn new(title: String) -> Self {
        let row = gtk::ListBoxRow::new();
        row.get_style_context().add_class("card");
        let description = Some(String::from("description"));

        let pixbuf = gdk_pixbuf::Pixbuf::new_from_file_at_scale(
            "data/resources/test-img.jpg",
            210,
            150,
            false,
        )
        .unwrap();

        let img = gtk::Image::new_from_pixbuf(Some(&pixbuf));
        img.get_style_context().add_class("img");

        let labels = Self::setup_labels();

        Self::setup_card(&row, &title, &description, &img, &labels);

        row.show_all();

        Card {
            row,
            title,
            description,
            img: Some(img),
            labels,
        }
    }

    fn setup_card(
        row: &gtk::ListBoxRow,
        title: &str,
        description: &Option<String>,
        img: &gtk::Image,
        labels: &gtk::Box,
    ) {
        row.set_widget_name("card"); // add css class card

        let card_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let title = gtk::Label::new(Some(title));
        title.set_max_width_chars(5);
        title.set_line_wrap(true);
        title.set_line_wrap_mode(pango::WrapMode::Char);
        let description: &str = &description.as_ref().unwrap();
        let description = gtk::Label::new(Some(description));

        card_box.add(img);
        card_box.set_child_packing(img, false, true, 5, gtk::PackType::Start);
        card_box.add(&title);
        //card_box.add(&description);
        card_box.add(labels);

        row.add(&card_box);
    }

    fn setup_labels() -> gtk::Box {
        let labels = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let first_label = gtk::Label::new(Some("first"));
        first_label.get_style_context().add_class("label");

        let second_label = gtk::Label::new(Some("second"));
        second_label.get_style_context().add_class("label");

        labels.add(&first_label);
        labels.add(&second_label);

        labels
    }
}

impl InnerWidget for Card {}
