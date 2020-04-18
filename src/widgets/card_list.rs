use gtk::prelude::*;
use gtk::WidgetExt;

use super::InnerWidget;

pub struct CardList {
    pub cards: gtk::ListBox,
    pub headerbar: gtk::ButtonBox,
    name: String,
    board_list_box: gtk::Box,
}

impl CardList {
    pub fn new(name: String) -> Self {
        let board_list_box = Self::setup_board_list();
        let headerbar = Self::setup_headerbar(&name);

        board_list_box.add(&headerbar);

        let sw = Self::setup_scrollbar(&board_list_box);
        board_list_box.add(&sw);

        let cards = gtk::ListBox::new();

        CardList {
            cards,
            headerbar,
            name,
            board_list_box,
        }
    }

    fn setup_headerbar(name: &str) -> gtk::ButtonBox {
        let headerbar = gtk::ButtonBox::new(gtk::Orientation::Horizontal);

        let list_name = gtk::Label::new(Some(name));
        list_name.set_halign(gtk::Align::Start);

        headerbar.add(&list_name);

        headerbar
    }

    fn setup_scrollbar(board_list: &gtk::Box) -> gtk::ScrolledWindow {
        let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);

        #[cfg(any(feature = "v3_22"))]
        scrolled_window.set_propagate_natural_width(true);
        scrolled_window.set_min_content_width(250);
        scrolled_window.set_vexpand_set(true);
        scrolled_window.set_vexpand(true);

        let viewport = Self::setup_viewport(board_list);

        scrolled_window.add(&viewport);

        scrolled_window
    }

    fn setup_viewport(board_list: &gtk::Box) -> gtk::Viewport {
        let viewport = gtk::Viewport::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        viewport.add(board_list);

        viewport
    }

    fn setup_board_list() -> gtk::Box {
        let board_list = gtk::Box::new(gtk::Orientation::Vertical, 5);

        let btn_add_card = gtk::Button::new_with_label("+");
        btn_add_card.set_margin_bottom(5);
        board_list.add(&btn_add_card);

        board_list.set_child_packing(&btn_add_card, false, true, 10, gtk::PackType::Start);
        board_list.show_all();

        board_list
    }
}

impl InnerWidget for CardList {}
