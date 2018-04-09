//! This is the API for Raven Website
//!
//! Is contains for now only the newsletter endpoints

#![feature(plugin)]
#![plugin(rocket_codegen)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#![cfg_attr(feature = "cargo-clippy", allow(doc_markdown))]
#![cfg_attr(feature = "cargo-clippy", allow(print_literal))]

// Used to load environment from .env file
extern crate dotenv;

// The web framework
extern crate rocket;

// The database
#[macro_use]
extern crate diesel;

// A generic connection pool and diesel wrapper
extern crate r2d2;
extern crate r2d2_diesel;

// Used for serialization and deserialization
#[macro_use]
extern crate serde_derive;
extern crate serde;

// Used for json
extern crate rocket_contrib;

// Used for random
extern crate rand;

pub mod catchers;
pub mod config;
pub mod db;
pub mod routes;

use config::{init_pool, AdminToken, MyConfig};

use catchers::get_catchers;
use routes::front::get_routes as front_routes;
use routes::user::get_routes as user_routes;

fn main() {
    dotenv::dotenv().ok();

    let config = MyConfig {
        pool: init_pool(&get_env("DATABASE_URL")),
        admin_token: AdminToken(get_env("ADMIN_TOKEN")),
    };

    rocket::ignite()
        .manage(config)
        .mount("/emails", user_routes())
        .mount("/", front_routes())
        .catch(get_catchers())
        .launch();
}

/// Retrieve needed environment variables or exit
fn get_env(var: &str) -> String {
    match std::env::var(var) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("error: the {} variable is not set.", var);
            std::process::exit(1);
        }
    }
}
