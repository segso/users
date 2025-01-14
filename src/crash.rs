use std::{fmt::Display, process};

pub fn crash(message: &str) -> ! {
    eprintln!("{message}");
    process::exit(1);
}

macro_rules! crash {
    ($($arg:tt)*) => {
        crash(&format!( $( $arg )* ))
    };
}
