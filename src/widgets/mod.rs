pub mod card_list;
pub mod main_stack;
pub mod window_headerbar;

pub trait InnerWidget {}

pub type WindowHeaderbar = window_headerbar::WindowHeaderbar;
pub type MainStack = main_stack::MainStack;
pub type CardList = card_list::CardList;
