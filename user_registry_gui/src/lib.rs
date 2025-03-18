mod app;
mod message;
mod page;
mod pages;
mod state;

use app::App;

pub fn run() -> iced::Result {
    iced::run(App::title, App::update, App::view)
}
