//! Contains all routes to serve front-end files

use rocket::response::{NamedFile, Redirect};
use rocket::Route;
use rocket_contrib::Template;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Returns all front-end routes
pub fn get_routes() -> Vec<Route> {
    routes![index, get]
}

/**
 * @api {get} /:path Get resource
 * @apiDescription Get resource from website for the given path
 * @apiName get
 * @apiGroup Website
 * @apiVersion 1.0.0
 *
 * @apiParam {String} path path of a resource
 *
 * @apiSuccess {text/html} page web page.
 *
 * @apiSuccessExample Success-Response:
 *     HTTP/1.1 200 OK
 *     Content-type text/html
 *
 * @apiErrorExample Error-Response:
 *     HTTP/1.1 404 Not found
 */
#[get("/<target..>")]
fn get(mut target: PathBuf) -> Result<Result<Option<NamedFile>, Template>, Redirect> {
    let path = Path::new("front").join(target.clone());
    if path.is_dir() {
        target.push("index.html");
        if let Some(path) = target.to_str() {
            Err(Redirect::to(path))
        } else {
            Err(Redirect::to("/"))
        }
    } else {
        let path_template = path.clone().with_extension("html.hbs");
        if path_template.exists() {
            if let Some(name) = extract_filename(target.clone().with_extension("html.hbs")) {
                return Ok(Err(template(name.to_string())));
            }
        }
        Ok(Ok(NamedFile::open(path).ok()))
    }
}

/**
 * @api {get} / Get website root
 * @apiDescription Get base of the website
 * @apiName index
 * @apiGroup Website
 * @apiVersion 1.0.0
 *
 * @apiSuccess {text/html} page web page.
 *
 * @apiSuccessExample Success-Response:
 *     HTTP/1.1 200 OK
 *     Content-type text/html
 */
#[get("/")]
fn index() -> Template {
    template("index".to_string())
}

/// Helper
fn template(name: String) -> Template {
    Template::render(name, HashMap::<&str, &str>::new())
}

/// Return filename without any extension
/// TODO: check unwrap
fn extract_filename(path: PathBuf) -> Option<String> {
    let s: String = path.to_str().unwrap().to_string();
    let (a, _) = s.split_at(s.find('.').unwrap_or(s.len()));
    Some(a.to_string())
}
