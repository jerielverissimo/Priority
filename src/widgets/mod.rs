pub mod main_stack;
pub mod window_headerbar;

pub trait InnerWidget {}

pub type WindowHeaderbar = window_headerbar::WindowHeaderbar;
pub type MainStack = main_stack::MainStack;
