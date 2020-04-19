use gtk::prelude::*;
use gtk::WidgetExt;

use super::InnerWidget;

#[derive(Clone)]
pub struct CardList {
    pub cards: gtk::ListBox,
    pub headerbar: gtk::ButtonBox,
    pub board_list_box: gtk::Box,
    pub btn_add_card: gtk::Button,
    name: String,
}

impl CardList {
    pub fn new(name: String) -> Self {
        let board_list_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let headerbar = Self::setup_headerbar(&name);

        board_list_box.add(&headerbar);

        let sw = Self::setup_scrollbar();
        board_list_box.add(&sw);

        let cards = gtk::ListBox::new();
        let viewport = Self::setup_viewport(&cards);
        sw.add(&viewport);

        let btn_add_card = Self::setup_button(&board_list_box);

        board_list_box.show_all();

        CardList {
            cards,
            headerbar,
            name,
            board_list_box,
            btn_add_card,
        }
    }

    fn setup_headerbar(name: &str) -> gtk::ButtonBox {
        let headerbar = gtk::ButtonBox::new(gtk::Orientation::Horizontal);

        let list_name = gtk::Label::new(Some(name));
        list_name.set_halign(gtk::Align::Start);

        headerbar.add(&list_name);

        headerbar
    }

    fn setup_scrollbar() -> gtk::ScrolledWindow {
        let sw = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        #[cfg(any(feature = "v3_22"))]
        sw.set_propagate_natural_width(true);
        sw.set_min_content_width(250);
        sw.set_vexpand_set(true);
        sw.set_vexpand(true);
        sw
    }

    fn setup_viewport(cards: &gtk::ListBox) -> gtk::Viewport {
        let viewport = gtk::Viewport::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        viewport.add(cards);

        viewport
    }

    fn setup_button(board_list: &gtk::Box) -> gtk::Button {
        let btn_add_card = gtk::Button::new_with_label("⁺");
        btn_add_card.set_margin_bottom(5);

        board_list.add(&btn_add_card);
        board_list.set_child_packing(&btn_add_card, false, true, 10, gtk::PackType::Start);

        btn_add_card
    }
}

impl InnerWidget for CardList {}
