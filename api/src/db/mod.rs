#[macro_use]
pub mod models;
mod schema;

use self::models::{NewUser, User};
use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use std::env;

// Connect to database
#[allow(dead_code)]
pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &MysqlConnection, user_email: &str) -> Result<User, String> {
    use self::schema::users;

    let new_user = NewUser {
        email: user_email,
        token: "test",
    };

    let row = insert_into(users::table).values(&new_user).execute(conn);
    match row {
        Ok(_) => Ok(users::table.order(users::id.desc()).first(conn).unwrap()),
        Err(s) => Err(s.to_string()),
    }
}

pub fn delete_user(
    conn: &MysqlConnection,
    user_email: &str,
    user_token: &str,
) -> Result<usize, String> {
    use self::schema::users::dsl::*;

    match users.filter(email.eq(user_email)).first::<User>(conn) {
        Err(_) => Err("Not found".to_string()),
        Ok(u) => {
            if u.token != user_token {
                return Err("Forbidden".to_string());
            }
            match delete(users.filter(email.eq(u.email))).execute(conn) {
                Ok(deleted_row) => Ok(deleted_row),
                Err(s) => Err(s.to_string()),
            }
        }
    }
}

pub fn get_all_users(conn: &MysqlConnection) -> Result<Vec<User>, String> {
    use self::schema::users::dsl::*;

    match users.load::<User>(conn) {
        Ok(u) => Ok(u),
        Err(s) => Err(s.to_string()),
    }
}

pub fn get_user(conn: &MysqlConnection, user_email: &str) -> Result<User, String> {
    use self::schema::users::dsl::*;

    match users.filter(email.eq(user_email)).first::<User>(conn) {
        Err(_) => Err("Not found".to_string()),
        Ok(user) => Ok(user),
    }
}
