use gtk::prelude::*;

use super::Card;
use super::CardList;
use super::InnerWidget;

#[derive(Clone)]
pub struct Board {
    button_add_list: gtk::Button,
    board_list: gtk::Box,
}

impl Board {
    pub fn new(builder: &gtk::Builder) -> Board {
        get_widget!(builder, gtk::Box, board_list);
        get_widget!(builder, gtk::Button, button_add_list);

        let board = Board {
            board_list,
            button_add_list,
        };

        board.setup_board(&builder);

        board
    }

    fn setup_board(&self, builder: &gtk::Builder) {
        let board = self;
        board
            .button_add_list
            .connect_clicked(clone!(@strong board, @weak builder => move |_| {

                get_widget!(builder, gtk::ButtonBox, button_box_add_list);
                let btn_position = board.board_list.get_child_position(&button_box_add_list);

                // TODO: Get name from user
                let list = CardList::new(String::from("New List"));

                list.board_list_box.set_margin_end(10);

                board.board_list
                    .pack_start(&list.board_list_box, false, false, 5);

                board.board_list.set_child_packing(
                    &list.board_list_box,
                    false,
                    true,
                    5,
                    gtk::PackType::Start,
                );

                board.board_list.set_child_position(&button_box_add_list, btn_position + 1);

                list.btn_add_card.connect_clicked(clone!(@strong list => move |_| {
                    // TODO: get card name from user
                    let card = Card::new(String::from("Test"));
                    list.cards.add(&card.row);
                    list.cards.show_all();
                    println!("Card Added!");
                }));
                println!("List Added!");
            }));
    }
}

impl InnerWidget for Board {}
