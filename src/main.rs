#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate website;

use dotenv::dotenv;
use std::env;
use std::process::{self};

use self::website::*;

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

#[get("/email/<email>")]
fn add_user(email: String) -> String {
    let connection = establish_connection();

    let user = create_user(&connection, &email);
    format!("Added in database: [id: {}, email: {}]", user.id, user.email)
}

fn main() {
    dotenv().ok();
    lazy_static::initialize(&DATABASE_URL);
    rocket::ignite().mount("/", routes![add_user]).launch();
}
