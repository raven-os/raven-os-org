//! Contains error catcher

use rocket;
use rocket::request::Request;
use rocket::Catcher;
use rocket_contrib::Template;
use std::collections::HashMap;

/// Returns all error catchers
pub fn get_catchers() -> Vec<Catcher> {
    errors![not_found]
}

#[error(404)]
fn not_found(_req: &Request) -> Template {
    //println!("{:?}", _req);
    Template::render("404", HashMap::<&str, &str>::new())
}
