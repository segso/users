use std::{
    io::{self, Write},
    path::Path,
};

use crate::{Data, User};

use super::data::read_data;

/// Writes a [`User`]'s details to the provided writer.
///
/// This function formats and writes the [`User`]'s `first_name`, `last_name`,
/// `email`, and `phone_number` along with their `id` to a writable destination,
/// such as a file or stdout.
///
/// Returns `Ok(())` on success or an `Err(io::Error)` if writing fails.
///
/// # Errors
/// This function can return an error if writing to the `writer` fails.
///
/// # Examples
/// ```rust
/// # use user_registry::{User, command::write_user};
/// let user = User {
///     first_name: "John".to_string(),
///     last_name: "Doe".to_string(),
///     email: "john@example.com".to_string(),
///     phone_number: "555-1234".to_string(),
/// };
///
/// let mut writer = Vec::new();
/// write_user(&user, 7, &mut writer).unwrap();
/// ```
pub fn write_user<W: Write>(user: &User, id: usize, writer: &mut W) -> Result<(), io::Error> {
    write!(
        writer,
        "User {id}:\n    First name: {}\n    Last name: {}\n    Email: {}\n    Phone number: {}\n",
        user.first_name, user.last_name, user.email, user.phone_number
    )
}

/// Displays user data from a [`Data`] and writes it to the provided writer.
///
/// This function sorts the [`User`]s by their ID, and writes the formatted
/// [`User`] details to the given writer using the [`write_user`] function. Each
/// [`User`]'s information is separated by a blank line.
///
/// Returns `Ok(())` if the user data is successfully written or an
/// `Err(io::Error)` if any error occurs while writing.
///
/// # Errors
/// This function can return an error if writing to the `writer` fails.
///
/// # Examples
/// ```ignore
/// # use user_registry::{command::show_data, Data, User};
/// let mut data = Data::new();
///
/// data.add_user(User {
///     first_name: "John".to_string(),
///     last_name: "Doe".to_string(),
///     email: "john@example.com".to_string(),
///     phone_number: "555-1234".to_string(),
/// });
///
/// let mut writer = Vec::new();
/// show_data(&data, &mut writer).unwrap();
/// ```
fn show_data<W: Write>(data: &Data, writer: &mut W) -> Result<(), io::Error> {
    let mut users = data.users().to_owned();
    users.sort_by_key(|(id, _)| *id);

    let mut first = true;

    for (id, user) in users {
        if first {
            first = false;
        } else {
            writeln!(writer)?;
        }

        write_user(user, id, writer)?;
    }

    Ok(())
}

/// Reads user data from a file at the specified `path` and writes it to the
/// provided writer.
///
/// This function reads the [`User`] data from the file at `path`, deserializes
/// it into a [`Data`] object, and then formats and writes the [`User`] details
/// to the given writer. Each [`User`]'s information is separated by a blank
/// line, and their details are written in a structured format.
///
/// Returns `Ok(())` if reading the data and writing the output is successful or
/// an `Err(io::Error)` if any error occurs while reading or writing.
///
/// # Errors
/// This function can return an error if reading from the file or writing to the
/// `writer` fails.
///
/// # Examples
/// ```rust
/// # use std::io::stdout;
/// # use user_registry::{command::show, Data};
/// fn show_from_file() {
///     let path = "path/to/data.json";
///     let mut writer = stdout();
///     show(path, &mut writer).unwrap();
/// }
/// ```
pub fn show<P: AsRef<Path>, W: Write>(path: P, writer: &mut W) -> Result<(), io::Error> {
    let data = read_data(&path)?;
    show_data(&data, writer)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a [`User`] with data for testing purposes.
    fn first_user() -> User {
        User {
            first_name: String::from("firstName"),
            last_name: String::from("firstSurname"),
            email: String::from("firstEmail"),
            phone_number: String::from("0123456789"),
        }
    }

    /// Helper function to create a [`User`] with data for testing purposes.
    fn second_user() -> User {
        User {
            first_name: String::from("secondName"),
            last_name: String::from("secondSurname"),
            email: String::from("secondEmail"),
            phone_number: String::from("9786543210"),
        }
    }

    /// Tests the behavior of writing a single [`User`] using the [`write_user`]
    /// function.
    ///
    /// This test ensures that:
    /// - The [`User`] data is correctly formatted and written to the buffer.
    /// - The output includes the correct [`User`] details.
    #[test]
    fn write_user_and_check_buffer() {
        let user = first_user();
        let mut writer = Vec::new();
        write_user(&user, 7, &mut writer).unwrap();

        assert_eq!(
            writer,
            b"User 7:
    First name: firstName
    Last name: firstSurname
    Email: firstEmail
    Phone number: 0123456789\n"
        );
    }

    /// Tests the behavior of displaying multiple [`User`]s using the `show_data`
    /// function.
    ///
    /// This test ensures that:
    /// - The [`User`] data is written correctly for multiple [`User`]s.
    /// - Each [`User`]'s details are separated by a blank line.
    /// - The [`User`]s are ordered by their ID.
    #[test]
    fn show_data_and_check_buffer() {
        let mut data = Data::new();
        data.add_user(first_user());
        data.add_user(second_user());

        let mut writer = Vec::new();
        show_data(&data, &mut writer).unwrap();

        assert_eq!(
            writer,
            b"User 0:
    First name: firstName
    Last name: firstSurname
    Email: firstEmail
    Phone number: 0123456789

User 1:
    First name: secondName
    Last name: secondSurname
    Email: secondEmail
    Phone number: 9786543210\n"
        );
    }
}
