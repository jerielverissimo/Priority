use gtk::prelude::*;

use super::InnerWidget;

pub struct MainStack {
    pub widget: gtk::Stack,
}

impl MainStack {
    pub fn new(builder: &gtk::Builder) -> Self {
        get_widget!(builder, gtk::Stack, main_stack);

        MainStack { widget: main_stack }
    }
}

impl InnerWidget for MainStack {}
