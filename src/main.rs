#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod models;
mod schema;

use dotenv::dotenv;
use std::env;
use std::process::{self};
use diesel::prelude::*;
use self::models::{User, NewUser};



// The different environment variables we are using.
//
// The idea here is to check at boot-time if the variable is set and not every time we
// need it.
lazy_static! {
    static ref DATABASE_URL: String = {
        match env::var("DATABASE_URL") {
            Ok(s) => s,
            Err(_) => {
                eprintln!("error: the DATABASE_URL variable is not set.");
                process::exit(1);
            }
        }
    };
}

// Connect to database
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// Add an user to database
#[get("/email/<email>")]
fn add_user(email: String) -> String {
    let connection = establish_connection();

    let user = create_user(&connection, &email);
    format!("Added in database: [id: {}, email: {}]", user.id, user.email)
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


fn main() {
    dotenv().ok();
    lazy_static::initialize(&DATABASE_URL);
    rocket::ignite().mount("/", routes![add_user]).launch();
}
