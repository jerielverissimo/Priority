use cairo;
use gtk::prelude::*;
use uuid::Uuid;

use super::InnerWidget;

#[derive(Clone)]
pub struct Card {
    pub row: gtk::ListBoxRow,
    pub title: String,
    pub description: Option<String>,
    pub id: Uuid,
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

        let id = Uuid::new_v4();

        Self::setup_card(&row, &title, &description, &img, &labels, &id);

        row.show_all();



        Card {
            row,
            title,
            description,
            id,
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
        id: &Uuid,
    ) {
        row.set_widget_name("card"); // add css class card

        let card_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let title = gtk::Label::new(Some(title));
        title.set_max_width_chars(5);
        title.set_line_wrap(true);
        title.set_line_wrap_mode(pango::WrapMode::Char);
        let description: &str = &description.as_ref().unwrap();
        let description = gtk::Label::new(Some(description));

        let handle = gtk::EventBox::new();
        handle.add(&card_box);

        card_box.add(img);
        card_box.set_child_packing(img, false, true, 5, gtk::PackType::Start);
        card_box.add(&title);
        card_box.add(&description);
        card_box.add(labels);

        let targets = vec![gtk::TargetEntry::new(
            "GTK_LIST_BOX_ROW",
            gtk::TargetFlags::SAME_APP,
            0,
        )];

        handle.drag_source_set(
            gdk::ModifierType::BUTTON1_MASK,
            &targets,
            gdk::DragAction::COPY,
        );

        handle.connect_drag_begin(move |w, ctx| {
            println!("Begin");
            let ww = w.get_allocated_width();
            let wh = w.get_allocated_height();
            let image = cairo::ImageSurface::create(cairo::Format::ARgb32, ww, wh).unwrap();
            let g = cairo::Context::new(&image);
            g.set_source_rgba(1.0, 1.0, 1.0, 0.8);
            g.rectangle(0.0,0.0,ww as f64, wh as f64);
            g.fill();

            w.draw(&g);

            ctx.drag_set_icon_surface(&image);
        });

        handle.connect_drag_data_get(clone!(@strong id => move |_, _, data, _, _| {
            println!("Got");
            data.set_text(&id.to_string());
        }));

        row.add(&handle);
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
