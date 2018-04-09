//! Contains error catcher

use rocket;
use rocket::Catcher;
use rocket::request::Request;
use rocket::response::NamedFile;
use std::path::Path;

/// Returns all error catchers
pub fn get_catchers() -> Vec<Catcher> {
    errors![not_found]
}

#[error(404)]
fn not_found(_req: &Request) -> Option<NamedFile> {
    //println!("{:?}", _req);
    NamedFile::open(Path::new("front/404.html")).ok()
}
