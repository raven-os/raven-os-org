//! Contains all routes to serve front-end files

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rocket::get;
use rocket::response::NamedFile;
use rocket_contrib::templates::Template;

#[get("/<files..>", rank = 0)]
pub fn static_files(files: PathBuf) -> Option<NamedFile> {
    let path = Path::new("static").join(files);
    if !path.is_dir() {
        NamedFile::open(path).ok()
    } else {
        None
    }
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", HashMap::<&str, &str>::new())
}

#[get("/logo")]
pub fn logo() -> Template {
    Template::render("logo", HashMap::<&str, &str>::new())
}
