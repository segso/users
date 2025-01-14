use std::{fs, io, path::Path};

pub mod data;
pub mod error;
pub mod write;

pub use data::{read_data, save_data};
pub use error::Error;
pub use write::{show, write_user};

use crate::User;

/// Adds a new [`User`] to the data file.
///
/// This function reads the existing [`Data`] from the file at the provided
/// `path`, adds the given [`User`], and then saves the updated [`Data`] back to
/// the file.
///
/// # Errors
/// This function may return an error if:
/// - Reading from the file fails (e.g., due to file access issues).
/// - Writing to the file fails (e.g., due to insufficient permissions).
///
/// # Examples
/// ```rust
/// # use user_registry::{command::add, User};
/// fn add_user() {
///     let user = User {
///         first_name: "John".to_string(),
///         last_name: "Doe".to_string(),
///         email: "john@example.com".to_string(),
///         phone_number: "555-1234".to_string(),
///     };
///     let path = "users.json";
///     add(path, user).unwrap();
/// }
/// ```
/// [`Data`]: crate::Data
pub fn add<P: AsRef<Path>>(path: P, user: User) -> Result<(), io::Error> {
    let mut data = read_data(&path)?;
    data.add_user(user);
    save_data(&path, &data)
}

/// Retrieves a [`User`] by their ID from the data file.
///
/// This function reads the [`Data`] from the file at the provided `path`,
/// searches for the [`User`] with the specified `id`, and returns it if found.
///
/// # Errors
/// This function returns an error if:
/// - Reading from the file fails (e.g., due to file access issues).
/// - The [`User`] with the given `id` is not found in the data file.
///
/// # Examples
/// ```rust
/// # use user_registry::{command::get, User};
/// fn get_user() {
///     let path = "users.json";
///     let user = get(path, 7).unwrap();
///     println!("Found user: {:?}", user);
/// }
/// ```
/// [`Data`]: crate::Data
pub fn get<P: AsRef<Path>>(path: P, id: usize) -> Result<User, Error> {
    let data = read_data(&path).map_err(Error::IoError)?;
    let user = data.user(id);

    user.map_or(Err(Error::UserNotFound(id)), |user| Ok(user.clone()))
}

/// Removes a [`User`] by their ID from the data file.
///
/// This function reads the [`Data`] from the file at the provided `path`,
/// removes the [`User`] with the specified `id`, and then saves the updated
/// [`Data`] back to the file.
///
/// # Errors
/// This function may return an error if:
/// - Reading from the file fails (e.g., due to file access issues).
/// - Writing to the file fails (e.g., due to insufficient permissions).
/// - The [`User`] with the specified `id` does not exist in the data file.
///
/// # Examples
/// ```rust
/// # use user_registry::{command::remove, User};
/// fn remove_user() {
///     let path = "users.json";
///     let removed_user = remove(path, 7).unwrap();
///     println!("Removed user: {:?}", removed_user);
/// }
/// ```
/// [`Data`]: crate::Data
pub fn remove<P: AsRef<Path>>(path: P, id: usize) -> Result<User, Error> {
    let mut data = read_data(&path).map_err(Error::IoError)?;
    let user = data.remove_user(id);
    save_data(&path, &data)?;

    user.ok_or(Error::UserNotFound(id))
}

/// Resets the data file by removing it.
///
/// This function deletes the file at the given `path`, effectively resetting
/// the [`Data`] stored in the file.
///
/// # Errors
/// This function may return an error if removing the file fails (e.g., the file
/// doesn't exist or there are permission issues).
///
/// # Examples
/// ```rust
/// # use user_registry::{command::reset};
/// fn reset_data() {
///     let path = "users.json";
///     reset(path).unwrap();
/// }
/// ```
/// [`Data`]: crate::Data
pub fn reset<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    fs::remove_file(path)
}
