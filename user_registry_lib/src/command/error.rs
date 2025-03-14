use std::{fmt::Display, io};

#[derive(Debug)]
pub enum Error {
    UserNotFound(usize),
    IoError(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserNotFound(id) => write!(f, "The user with the ID {id} was not found."),
            Self::IoError(err) => write!(f, "I/O error: {err}"),
        }
    }
}

impl std::error::Error for Error {}
