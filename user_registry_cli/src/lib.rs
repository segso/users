use std::{fs, io::stdout};

use app::{App, Command};
use clap::Parser;
use user_registry_lib::{
    User,
    command::{add, get, remove, reset, show, write_user},
};

mod app;

pub fn run() -> Result<(), String> {
    let app = App::parse();
    let data_file = app.get_data().ok_or(String::from(
        "Couldn't get data file path. Try using --data to specify one.",
    ))?;

    let Some(parent) = data_file.parent() else {
        return Err(String::from("Couldn't get data file parent."));
    };

    if let Err(err) = fs::create_dir_all(parent) {
        return Err(format!("Failed to create data directory: {err}"));
    }

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
        Command::Gui => {
            #[cfg(not(feature = "gui"))]
            return Err(String::from(
                "The 'gui' feature is disabled. To enable it, recompile the program with the flag `--features gui`.",
            ));

            #[cfg(feature = "gui")]
            if let Err(err) = user_registry_gui::run(&data_file) {
                return Err(format!("An error occurred in the GUI: {err}"));
            }
        }
    }

    Ok(())
}
