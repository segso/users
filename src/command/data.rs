use crate::Data;

use std::{fs, io, path::Path};

/// Reads [`User`] data from a file at the specified `path` and returns it as a
/// [`Data`] object.
///
/// This function attempts to read the contents of the file at the given `path`.
/// If the file exists, it reads the contents and deserializes the JSON data
/// into a [`Data`] object. If the file does not exist or is empty, it returns a
/// default [`Data`] object.
///
/// If the file is successfully read and parsed, it returns `Ok(Data)` but if an
/// error occurs (e.g., file reading or JSON parsing), it returns an
/// `Err(io::Error)`.
///
/// # Errors
/// This function may return an `Err(io::Error)` if reading from the file or
/// deserializing the contents fails.
///
/// # Examples
/// ```rust
/// # use std::io::Error;
/// # use user_registry::{command::read_data, Data};
/// fn read() -> Data {
///     let data = read_data("path/to/data.json").unwrap();
///     data
/// }
/// ```
/// [`User`]: crate::User
pub fn read_data<P: AsRef<Path>>(path: P) -> Result<Data, io::Error> {
    let contents = if path.as_ref().exists() {
        fs::read_to_string(&path)?
    } else {
        String::new()
    };

    Ok(if contents.is_empty() {
        Data::default()
    } else {
        serde_json::from_str(&contents)?
    })
}

/// Saves [`User`] data to a file at the specified `path`.
///
/// This function serializes the given [`Data`] object to a JSON string and
/// writes it to the file at the given `path`. If the file already exists, it
/// will be overwritten. If the file doesn't exist, it will be created.
///
/// If the file is successfully written, it returns `Ok(())`. If an error occurs
/// (e.g., file writing or serialization failure), it returns an
/// `Err(io::Error)`.
///
/// # Errors
/// This function may return an `Err(io::Error)` if writing to the file or
/// serializing the [`Data`] fails.
///
/// # Examples
/// ```rust
/// # use user_registry::{command::save_data, Data};
/// fn save() {
///     let data = Data::new();
///     save_data("path/to/data.json", &data).unwrap();
/// }
/// ```
/// [`User`]: crate::User
pub fn save_data<P: AsRef<Path>>(path: P, data: &Data) -> Result<(), io::Error> {
    fs::write(path, serde_json::to_string(&data)?)
}
