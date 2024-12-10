use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Store a new user entry in the file.
    Add {
        /// The user's first name.
        first_name: String,

        /// The user's surname (last name).
        last_name: String,

        /// The user's email address.
        email: String,

        /// The user's telephone number.
        phone_number: u64,
    },

    /// Retrieve a user's data by their unique ID.
    Get {
        /// The ID of the user whose data is to be fetched.
        id: usize,
    },

    /// Remove a user entry from the file.
    Remove {
        /// The ID of the user to remove.
        id: usize,
    },

    /// Permanently delete all user data.
    Reset,

    /// Display all user data in JSON format.
    Show,
}

/// Program to register users in a file with their data via GUI or CLI.
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct App {
    /// File to load and save user data (defaults to data directory).
    #[arg(short, long, value_name = "FILE")]
    pub data: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

impl App {
    pub fn get_data(&self) -> Option<PathBuf> {
        self.data.clone().or_else(|| {
            dirs::data_dir().map(|mut path| {
                path.push("users_registry");
                path.push("users.txt");
                path
            })
        })
    }
}
