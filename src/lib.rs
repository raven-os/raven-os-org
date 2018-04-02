#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{User, NewUser};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(conn: &MysqlConnection, email: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        email: email
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");

    users::table.order(users::id.desc()).first(conn).unwrap()
}
