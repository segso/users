mod cli;
pub mod command;
mod data;
mod user;

pub use cli::{App, Command};
pub use data::Data;
pub use user::User;

use std::io::stdout;

use clap::Parser;

use command::{add, get, remove, reset, show, write_user};

pub fn run() -> Result<(), String> {
    let app = App::parse();
    let data_file = app.get_data().ok_or(String::from(
        "Couldn't get data file path. Try using --data to specify one.",
    ))?;

    if data_file.exists() && !data_file.is_file() {
        return Err(String::from(
            "The data path must be a file. Specify another one.",
        ));
    }

    match app.command {
        Command::Add {
            first_name,
            last_name,
            email,
            phone_number,
        } => {
            let user = User {
                first_name,
                last_name,
                email,
                phone_number: phone_number.to_string(),
            };

            add(data_file, user).map_err(|err| format!("User couldn't be added: {err}"))?;
        }
        Command::Get { id } => {
            let user = get(data_file, id).map_err(|err| format!("Couldn't get user: {err}"))?;
            write_user(&user, id, &mut stdout())
                .map_err(|err| format!("Couldn't write user: {err}"))?;
        }
        Command::Remove { id } => {
            remove(data_file, id).map_err(|err| format!("Couldn't remove user: {err}"))?;
        }
        Command::Reset => {
            reset(data_file).map_err(|err| format!("Couldn't reset the data file: {err}"))?;
        }
        Command::Show => {
            show(data_file, &mut stdout()).map_err(|err| format!("Couldn't write users: {err}"))?;
        }
    }

    Ok(())
}
