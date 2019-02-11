//! This is the API for Raven-OS's landing page
//!
//! Is contains for now only the newsletter endpoints

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

pub mod app;
pub mod db;
pub mod routes;

use rocket;
use crate::app::newsletter::Newsletter;
use crate::app::App;

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
            rocket::routes![
                routes::frontend::static_files,
                routes::frontend::index,
                routes::frontend::logo,
            ],
        )
        .mount(
            "/newsletter/",
            rocket::routes![
                routes::newsletter::add,
                routes::newsletter::remove,
                routes::newsletter::dump,
            ],
        )
        .register(rocket::catchers![routes::error::not_found])
        .attach(rocket_contrib::templates::Template::fairing())
        .launch();
}
