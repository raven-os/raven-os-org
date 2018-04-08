//! Contains all routes to serve front-end files

use rocket::Route;
use rocket::response::{NamedFile, Redirect};
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
fn get(mut target: PathBuf) -> Result<Option<NamedFile>, Redirect> {
    let path = Path::new("front").join(target.clone());
    if path.is_dir() {
        target.push("index.html");
        if let Some(path) = target.to_str() {
            Err(Redirect::to(path))
        } else {
            Err(Redirect::to("/"))
        }
    } else {
        Ok(NamedFile::open(path).ok())
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
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("front/index.html")).ok()
}
