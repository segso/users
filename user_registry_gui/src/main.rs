use std::{fs, process};

use user_registry_gui::run;

pub fn main() {
    let data_file = dirs::data_dir()
        .map(|mut path| {
            path.push("users_registry");
            path.push("users.json");
            path
        })
        .unwrap_or_else(|| {
            eprintln!("Couldn't get data file path. Try using --data to specify one.");
            process::exit(1);
        });

    let parent = data_file.parent().unwrap_or_else(|| {
        eprintln!("Couldn't get data file parent.");
        process::exit(1);
    });

    if let Err(err) = fs::create_dir_all(parent) {
        eprintln!("Failed to create data directory: {err}");
        process::exit(1);
    }

    if data_file.exists() && !data_file.is_file() {
        eprintln!("The data path must be a file. Specify another one.");
        process::exit(1);
    }

    if let Err(err) = run(&data_file) {
        eprintln!("An error occurred in the GUI: {err}");
        process::exit(1);
    }
}
