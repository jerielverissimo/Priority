pub mod board;
pub mod card;
pub mod card_list;
pub mod main_stack;
pub mod window_headerbar;

pub trait InnerWidget {}

pub type Board = board::Board;
pub type Card = card::Card;
pub type CardList = card_list::CardList;
pub type MainStack = main_stack::MainStack;
pub type WindowHeaderbar = window_headerbar::WindowHeaderbar;
