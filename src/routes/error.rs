//! Contains error catcher

use std::collections::HashMap;

use rocket::request::Request;
use rocket_contrib::Template;

#[catch(404)]
pub fn not_found(_req: &Request) -> Template {
    Template::render("404", HashMap::<&str, &str>::new())
}
