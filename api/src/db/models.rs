//! Contains objects that store data
//!
//! Data are stored in struct for database interaction
//! such as retrieve or insert.

use super::schema::users;

// User store retrieved data from the user table
#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub token: String,
}

// NewUser store data that will be inserted into the database
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub token: &'a str,
}
