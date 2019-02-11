//! This module contains structures used to represent all data stored in database
//! by the newsletter system.

use crate::db::newsletter::schema::newsletter_users;
use serde::{Deserialize, Serialize};

/// Represents a user suscribed to the newsletter
#[derive(
    Queryable, Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default,
)]
pub struct NewsletterUser {
    id: i32,
    email: String,
    token: String,
}

impl NewsletterUser {
    /// Return the user's id
    pub fn id(&self) -> &i32 {
        &self.id
    }

    /// Return the user's email
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Return the user's token
    pub fn token(&self) -> &str {
        &self.token
    }
}

/// NewsletterUser stores the data that will be inserted into the database
#[derive(Insertable, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[table_name = "newsletter_users"]
pub struct NewNewsletterUser<'a> {
    pub email: &'a str,
    pub token: &'a str,
}

impl<'a> NewNewsletterUser<'a> {
    /// Return the user's email
    pub fn email(&self) -> &'a str {
        &self.email
    }

    /// Return the user's token
    pub fn token(&self) -> &'a str {
        &self.token
    }
}
