use std::{
    error,
    fmt::{self, Display, Formatter},
    io,
};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    IcedError(iced::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<iced::Error> for Error {
    fn from(err: iced::Error) -> Self {
        Self::IcedError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(err) => write!(f, "IO Error: {err}"),
            Self::IcedError(err) => write!(f, "Iced Error: {err}"),
        }
    }
}

impl error::Error for Error {}
