#[macro_use]
mod models;
mod schema;

use std::env;
use diesel::prelude::*;
use self::models::{User, NewUser};
use diesel::insert_into;
use diesel::delete;

// Connect to database
#[allow(dead_code)]
pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(conn: &MysqlConnection, email: &'a str) -> User {
    use self::schema::users;

    let new_user = NewUser {
        email: email,
        token: "test"
    };

    insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");

    User {email: new_user.email.to_string(), token: new_user.token.to_string()}
}

pub fn delete_user(conn: &MysqlConnection, user_email: &str, user_token: &str) -> () {
    use self::schema::users::dsl::*;

    delete(users.filter(email.eq(user_email))
                .filter(token.eq(user_token)))
        .execute(conn)
        .expect("Error deleting");
}

#[allow(dead_code)]
pub fn get_all_users(conn: &MysqlConnection) -> Vec<User> {
    use self::schema::users::dsl::*;

    let results = users.load::<User>(conn)
        .expect("Error loading users");
    results
}
