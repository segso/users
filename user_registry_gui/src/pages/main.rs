use iced::{Element, widget::text};

use crate::message::Message;

pub fn title() -> String {
    String::from("Main Page")
}

pub fn update() {}

pub fn view<'a>() -> Element<'a, Message> {
    text("Main Page").into()
}
