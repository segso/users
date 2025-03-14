use serde::{Deserialize, Serialize};

/// Represents a user with basic contact information.
///
/// This struct stores the user's telephone number, first name, last name, and
/// email address.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    /// The user's first name.
    #[serde(rename = "n")]
    pub first_name: String,

    /// The user's surname (last name).
    #[serde(rename = "s")]
    pub last_name: String,

    /// The user's email address.
    #[serde(rename = "e")]
    pub email: String,

    /// The user's telephone number.
    #[serde(rename = "p")]
    pub phone_number: String,
}
