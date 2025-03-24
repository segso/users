mod app;
mod error;
mod message;
mod page;
mod pages;
mod state;

use app::App;
pub use error::Error;

use std::path::Path;

pub fn run<P: AsRef<Path>>(_data_file: P) -> Result<(), Error> {
    iced::run(App::title, App::update, App::view)?;

    Ok(())
}
