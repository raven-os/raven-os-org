//! This is the API for Raven Website
//!
//! Is contains for now only the newsletter endpoints

#![feature(plugin)]
#![plugin(rocket_codegen)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#![cfg_attr(feature = "cargo-clippy", allow(doc_markdown))]
#![cfg_attr(feature = "cargo-clippy", allow(print_literal))]

extern crate dotenv;
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate rand;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json as json;
extern crate uuid;

pub mod app;
pub mod db;
pub mod routes;

use app::newsletter::Newsletter;
use app::App;

/// Retrieves the needed environment variables or exits
fn get_env(var: &str) -> String {
    match std::env::var(var) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("error: the \"{}\" environment variable is not set.", var);
            std::process::exit(1);
        }
    }
}

fn main() {
    dotenv::dotenv().ok();

    let app = App::from(&get_env("DATABASE_URL")).expect("Failed to start app");
    let newsletter = Newsletter::new(get_env("RAVEN_ADMIN_TOKEN"));

    rocket::ignite()
        .manage(app)
        .manage(newsletter)
        .mount(
            "/",
            routes![
                routes::frontend::static_files,
                routes::frontend::index,
                routes::frontend::logo,
            ],
        )
        .mount(
            "/newsletter/",
            routes![
                routes::newsletter::add,
                routes::newsletter::remove,
                routes::newsletter::dump,
            ],
        )
        .catch(errors![routes::error::not_found,])
        .attach(rocket_contrib::Template::fairing())
        .launch();
}
