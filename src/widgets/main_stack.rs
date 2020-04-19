use gtk::prelude::*;

use super::Board;
use super::InnerWidget;

pub struct MainStack {
    pub widget: gtk::Stack,
    current_board: Board,
}

impl MainStack {
    pub fn new(builder: &gtk::Builder) -> Self {
        get_widget!(builder, gtk::Stack, main_stack);
        let current_board = Board::new(builder);

        MainStack {
            widget: main_stack,
            current_board,
        }
    }
}

impl InnerWidget for MainStack {}
