//! Contains objects that store data
//!
//! Data are stored in struct for database interaction
//! such as retrieve or insert.

use super::schema::users;

/// User struct store retrieved data from the user table
#[derive(Queryable, Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
         Default)]
pub struct User {
    id: i32,
    email: String,
    token: String,
}

/// Implementation of functions for User struct
impl User {
    /// Return the id
    pub fn id(&self) -> &i32 {
        &self.id
    }

    /// Return the email
    pub fn email(&self) -> &String {
        &self.email
    }

    /// Return the token
    pub fn token(&self) -> &String {
        &self.token
    }
}

/// NewUser store data that will be inserted into the database
#[derive(Insertable, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub token: &'a str,
}

/// Implementation of functions for NewUser struct
impl<'a> NewUser<'a> {
    /// Return the email
    pub fn email(&self) -> &'a str {
        &self.email
    }

    /// Return the token
    pub fn token(&self) -> &'a str {
        &self.token
    }
}
