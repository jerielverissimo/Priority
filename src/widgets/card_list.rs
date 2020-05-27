use gtk::prelude::*;
use gtk::WidgetExt;
use uuid::Uuid;

use std::collections::HashMap;

use super::InnerWidget;
use super::Card;

#[derive(Clone)]
pub struct CardList {
    pub cards: HashMap<Uuid, Card>,
    pub list: gtk::ListBox,
    pub headerbar: gtk::ButtonBox,
    pub board_list_box: gtk::Box,
    pub btn_add_card: gtk::Button,
    pub widget: gtk::EventBox,
    name: String,
}

impl CardList {
    pub fn new(name: String) -> Self {
        let board_list_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let headerbar = Self::setup_headerbar(&name);

        board_list_box.add(&headerbar);

        let sw = Self::setup_scrollbar();
        board_list_box.add(&sw);

        let list = gtk::ListBox::new();
        let widget = gtk::EventBox::new();

        widget.add(&list);
        list.get_style_context().add_class("list");
        let viewport = Self::setup_viewport(&widget);
        sw.add(&viewport);

        let btn_add_card = Self::setup_btn_add_card();

        list.insert(&btn_add_card, -1);
        list.set_sort_func(Some(box |head, tail| head.get_index() - tail.get_index()));

        list.set_selection_mode(gtk::SelectionMode::None);


        board_list_box.show_all();

        let cards: HashMap<Uuid, Card> = HashMap::new();

        CardList {
            cards,
            list,
            headerbar,
            name,
            widget,
            board_list_box,
            btn_add_card,
        }
    }

    pub fn add(&mut self, card: Card) {
        self.list.add(&card.row);
        self.cards.insert(card.id, card);
        self.list.show_all();
    }

    pub fn remove(&mut self, id: &Uuid) -> Option<Card> {
        let card = self.cards.get(id);
        self.list.remove(&card.unwrap().row);
        self.cards.remove(id)
    }

    fn setup_headerbar(name: &str) -> gtk::ButtonBox {
        let headerbar = gtk::ButtonBox::new(gtk::Orientation::Horizontal);

        let list_name = gtk::Label::new(Some(name));
        list_name.set_justify(gtk::Justification::Left);
        list_name.set_halign(gtk::Align::Start);
        #[cfg(any(feature = "v3_22"))]
        list_name.set_xalign(0.0);
        list_name.set_valign(gtk::Align::Start);

        headerbar.add(&list_name);
        headerbar.set_child_packing(&list_name, false, true, 5, gtk::PackType::Start);

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

    fn setup_viewport(list: &gtk::EventBox) -> gtk::Viewport {
        let viewport = gtk::Viewport::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        viewport.add(list);

        viewport
    }

    fn setup_btn_add_card() -> gtk::Button {
        let btn_add_card = gtk::Button::new();
        let label = gtk::Label::new(Some("+"));

        label.set_halign(gtk::Align::Center);
        label.set_valign(gtk::Align::Center);
        label.set_justify(gtk::Justification::Center);

        btn_add_card.add(&label);

        btn_add_card.set_halign(gtk::Align::Center);
        btn_add_card.set_valign(gtk::Align::Center);
        btn_add_card.get_style_context().add_class("btn-add");

        btn_add_card
    }

    pub fn connect_drop<F: Fn(Uuid) + 'static>(&self, widget: &gtk::EventBox, cb: F) {
        let targets = vec![gtk::TargetEntry::new(
            "GTK_LIST_BOX_ROW",
            gtk::TargetFlags::SAME_APP,
            0,
        )];

        widget.drag_dest_set(gtk::DestDefaults::empty(), &targets, gdk::DragAction::all());
        widget.drag_dest_add_text_targets();

        widget.connect_drag_motion(|_, ctx, _, _, time| {
            println!("Motion");
            ctx.drag_status(gdk::DragAction::MOVE, time);
            glib::signal::Inhibit(true)
        });

        widget.connect_drag_drop(|w, ctx, _, _, time| {
            println!("Drop");
            println!(">>> WIDGET {:?}:", w);
            if let Some(target) = w.drag_dest_find_target(ctx, None) {
                w.drag_get_data(ctx, &target, time);
            }
            glib::signal::Inhibit(true)
        });

        widget.connect_drag_data_received(move |_, _, _, _, data, _, _| {
            println!("Dragging received!!!");
            println!(">>> DATA: {:?}", data);
            if let Some(_card_id) = data.get_text() {}
        });
    }
}

impl InnerWidget for CardList {}
