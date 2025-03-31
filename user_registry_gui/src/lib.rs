mod app;
mod error;
mod message;
mod page;
mod pages;
mod state;

use app::App;
pub use error::Error;
use state::State;

use iced::Task;

use std::path::Path;

pub fn run<P: AsRef<Path>>(data_file: P) -> Result<(), Error> {
    let state = State::with_data_file(data_file)?;

    iced::application(App::title, App::update, App::view).run_with(|| (state, Task::none()))?;

    Ok(())
}
