use iced::Element;

use crate::{message::Message, page::Page, pages, state::State};

pub struct App;

impl App {
    pub fn title(state: &State) -> String {
        match state.page {
            Page::Main => pages::main::title(),
        }
    }

    pub fn update(state: &mut State, _: Message) {
        match state.page {
            Page::Main => pages::main::update(),
        }
    }

    pub fn view(state: &State) -> Element<Message> {
        match state.page {
            Page::Main => pages::main::view(),
        }
    }
}
