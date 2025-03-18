use std::process;

use user_registry_gui::run;

pub fn main() {
    if let Err(err) = run() {
        eprintln!("An error occurred in the GUI: {err}");
        process::exit(1);
    }
}
