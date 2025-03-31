use std::{io, path::Path};

use user_registry_lib::{Data, command::read_data};

use crate::page::Page;

#[derive(Default)]
pub struct State {
    pub page: Page,
    pub data: Data,
}

impl State {
    pub fn with_data_file<P: AsRef<Path>>(data_file: P) -> Result<Self, io::Error> {
        let data = read_data(data_file)?;

        Ok(Self {
            data,
            ..Default::default()
        })
    }
}
