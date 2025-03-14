use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::User;

/// A structure that stores a collection of [users] with their associated
/// contact information.
///
/// This structure maintains a map of users identified by a unique numeric ID.
/// It also tracks the next available ID to assign to a new user, ensuring each
/// user gets a unique identifier.
///
/// # Examples
/// ```rust
/// # use user_registry_lib::{Data, User};
/// #
/// let mut data = Data::new();
/// let user = User {
///     first_name: "John".to_string(),
///     last_name: "Doe".to_string(),
///     email: "john@example.com".to_string(),
///     phone_number: "555-1234".to_string(),
/// };
/// let id = data.add_user(user.clone());
/// let retrieved_user = data.user(id);
///
/// assert_eq!(retrieved_user, Some(&user));
/// ```
///
/// [users]: User
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Data {
    /// The next available unique ID to be assigned to a user.
    #[serde(rename = "i")]
    next_id: usize,

    /// A map of user IDs to their associated [`User`] details.
    #[serde(rename = "u")]
    users: HashMap<usize, User>,
}

impl Data {
    /// Creates a new, empty `Data` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the `next_id` to the smallest integer that is not used as key in the
    /// `users` map.
    ///
    /// This method iterates over natural numbers to find the smallest usable ID for
    /// the map. This is designed to generate unique user IDs in situations where
    /// IDs are assigned sequentially and may have gaps due to deletions or other
    /// reasons.
    ///
    /// # Example
    /// ```ignore
    /// let mut data = Data::new();
    /// data.calculate_next_id();
    /// assert_eq!(data.next_id, 0);
    ///
    /// let user = User {
    ///     first_name: "John".to_string(),
    ///     last_name: "Doe".to_string(),
    ///     email: "john@example.com".to_string(),
    ///     phone_number: "555-1234".to_string(),
    /// };
    ///
    /// data.add_user(user);
    /// data.calculate_next_id();
    /// assert_eq!(data.next_id, 1);
    /// ```
    fn calculate_next_id(&mut self) {
        let mut next_id = 0;

        while self.users.contains_key(&next_id) {
            next_id += 1;
        }

        self.next_id = next_id;
    }

    /// Adds a new user to the `Data` structure.
    ///
    /// This method assigns a unique ID to the given user and adds them to the
    /// collection. If the user is successfully added, it returns the assigned ID.
    /// If the user cannot be added, it returns [`None`].
    ///
    /// # Examples
    /// ```rust
    /// # use user_registry_lib::{Data, User};
    /// let mut data = Data::new();
    /// let user = User {
    ///     first_name: "John".to_string(),
    ///     last_name: "Doe".to_string(),
    ///     email: "john@example.com".to_string(),
    ///     phone_number: "555-1234".to_string(),
    /// };
    /// let user_id = data.add_user(user);
    ///
    /// assert_eq!(user_id, 0);
    /// ```
    pub fn add_user(&mut self, user: User) -> usize {
        let id = self.next_id;
        self.users.insert(id, user);
        self.calculate_next_id();
        id
    }

    /// Retrieves a user by their ID.
    ///
    /// This method looks up a user by their unique ID. If the user exists, it
    /// returns a reference to the user's information; otherwise, it returns
    /// [`None`].
    ///
    /// # Examples
    /// ```rust
    /// # use user_registry_lib::Data;
    /// let data = Data::new();
    /// let user = data.user(1);
    /// assert_eq!(user, None);
    /// ```
    pub fn user(&self, id: usize) -> Option<&User> {
        self.users
            .iter()
            .find(|(user_id, _)| **user_id == id)
            .map(|(_, user)| user)
    }

    /// Removes a user by their ID.
    ///
    /// This method removes a user from the collection by their unique ID. It
    /// returns the removed user if found, or [`None`] if no user exists with the
    /// given ID.
    ///
    /// # Examples
    /// ```rust
    /// # use user_registry_lib::Data;
    /// let mut data = Data::new();
    /// let removed_user = data.remove_user(1);
    /// assert_eq!(removed_user, None);
    /// ```
    pub fn remove_user(&mut self, id: usize) -> Option<User> {
        let user = self.users.remove(&id);
        self.calculate_next_id();
        user
    }

    /// Resets the collection, clearing all users.
    ///
    /// This method clears all users from the collection and resets the ID counter.
    /// It returns `true` if the collection was non-empty before the reset, or
    /// `false` if the collection was already empty.
    ///
    /// # Examples
    /// ```rust
    /// # use user_registry_lib::Data;
    /// let mut data = Data::new();
    /// let reset_result = data.reset();
    /// assert_eq!(reset_result, false);
    /// ```
    pub fn reset(&mut self) -> bool {
        if self.users.is_empty() {
            return false;
        }

        self.users.clear();
        self.calculate_next_id();
        true
    }

    /// Retrieves all users in the collection.
    ///
    /// This method returns all users as a `Vec` of tuples, where each tuple
    /// contains the user's ID and a reference to the [`User`].
    ///
    /// # Examples
    /// ```rust
    /// # use user_registry_lib::Data;
    /// let data = Data::new();
    /// let all_users = data.users();
    /// assert!(all_users.is_empty());
    /// ```
    pub fn users(&self) -> Vec<(usize, &User)> {
        self.users.iter().map(|(id, user)| (*id, user)).collect()
    }
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

    /// Tests adding and removing [`User`]s from the `Data` structure.
    ///
    /// This test ensures that:
    /// - [`User`]s are properly added to the `Data` structure with incremental IDs.
    /// - [`User`]s can be removed successfully by their ID.
    /// - Attempting to remove a [`User`] twice returns [`None`].
    #[test]
    fn add_and_remove_users() {
        let mut data = Data::new();

        // Add the first user and check the assigned ID.
        let first_id = data.add_user(first_user());
        assert_eq!(first_id, 0);

        // Add the second user and check the assigned ID.
        let second_id = data.add_user(second_user());
        assert_eq!(second_id, 1);

        // Check that users are removed correctly.
        assert_eq!(data.remove_user(first_id), Some(first_user()));
        assert_eq!(data.remove_user(first_id), None);
        assert_eq!(data.remove_user(second_id), Some(second_user()));
    }

    /// Tests adding and retrieving [`User`]s by ID from the `Data` struct.
    ///
    /// This test ensures that:
    /// - [`User`]s get correct IDs.
    /// - Getting a [`User`] via ID returns the correct data.
    #[test]
    fn get_user() {
        let mut data = Data::new();

        // Add the first user and check the ID.
        let first_id = data.add_user(first_user());
        assert_eq!(first_id, 0);

        // Add the second user and check the ID.
        assert_eq!(data.add_user(second_user()), 1);

        // Retrieve and check the first user using their ID.
        assert_eq!(data.user(first_id), Some(&first_user()));
    }

    /// Tests retrieving all [`User`]s and resetting the `Data` struct.
    ///
    /// This test ensures that:
    /// - Getting all the [`User`]s returns the correct data.
    /// - Resetting the `Data` clears all the registered [`User`]s.
    /// - After a reset, the next internal id goes back to zero.
    #[test]
    fn get_users_and_reset_data() {
        let mut data = Data::new();

        // Add users to the data.
        data.add_user(first_user());
        data.add_user(second_user());

        // Verify that returned users are correct.
        let mut users = data.users();
        users.sort_by_key(|(id, _)| *id);
        assert_eq!(users, &[(0, &first_user()), (1, &second_user())]);

        // Check that next_id returns to zero.
        assert_eq!(data.next_id, 2);
        assert!(data.reset());
        assert_eq!(data.next_id, 0);

        // Ensure no users remain after resetting.
        assert!(data.users().is_empty());

        // Ensure resetting again does not change the data.
        assert!(!data.reset());
        assert!(data.users().is_empty());
    }

    /// Tests the behavior of removing a [`User`] and updating the next available ID
    /// in the `Data` structure.
    ///
    /// This test ensures that:
    /// - When a [`User`] is added, the correct ID is assigned
    /// - When a [`User`] is removed, the `next_id` is updated accordingly.
    #[test]
    fn remove_and_check_next_id() {
        let mut data = Data::new();

        // Add first user
        data.add_user(first_user());
        // Add second user and capture the ID assigned
        let id = data.add_user(second_user());
        // Add first user again
        data.add_user(first_user());

        // Verify that the second user has been added and can be retrieved by ID
        assert_eq!(data.user(id), Some(&second_user()));
        // Verify that the next available ID is 3 after adding 3 users
        assert_eq!(data.next_id, 3);

        // Remove the second user by ID and verify removal
        assert_eq!(data.remove_user(id), Some(second_user()));
        // Verify that the next available ID is set back to the ID of the removed user
        assert_eq!(data.next_id, id);
    }
}
