//! This is the API for Raven Website
//!
//! Is contains for now only the newsletter endpoints

#![feature(plugin)]
#![plugin(rocket_codegen)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#![cfg_attr(feature = "cargo-clippy", allow(doc_markdown))]

extern crate dotenv;
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;

extern crate serde;

extern crate rocket_contrib;

use std::process;

use rocket::http::Status;

use rocket_contrib::Json;

use rocket::response::status::Custom;

pub mod config;
mod db;

use config::{init_pool, AdminToken, Config, DbConn};

use db::models::User;

/// Retrieve needed environment variables or exit
fn get_env(var: &str) -> String {
    match std::env::var(var) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("error: the {} variable is not set.", var);
            process::exit(1);
        }
    }
}

fn main() {
    dotenv::dotenv().ok();

    let database_url = get_env("DATABASE_URL");
    let admin_token = get_env("ADMIN_TOKEN");

    let config = Config {
        pool: init_pool(&database_url),
        admin_token: AdminToken(admin_token),
    };

    rocket::ignite()
        .manage(config)
        .mount(
            "/emails",
            routes![add_user, remove_user, get_users, get_user],
        )
        .launch();
}

/* ==================== ROUTE HANDLER ==================== */

/**
 * @api {post} /emails/:email Add email
 * @apiName AddEmail
 * @apiGroup emails
 * @apiVersion 1.0.0
 *
 * @apiParam {String} email User's email.
 *
 * @apiSuccess {Integer} id     User's id.
 * @apiSuccess {String}  email  User's email.
 * @apiSuccess {String}  token  User's token.
 *
 * @apiSuccessExample Success-Response:
 *     HTTP/1.1 200 OK
 *     {
 *       "id": 42,
 *       "email": "raven@os.com",
 *       "token": "azerty4242"
 *     }
 *
 * @apiErrorExample Error-Response:
 *     HTTP/1.1 400 Bad Request
 *     "Already registered"
 */
#[post("/<email>")]
fn add_user(connection: DbConn, email: String) -> Result<Custom<Json<User>>, Custom<String>> {
    match db::create_user(&connection, &email) {
        Ok(user) => Ok(Custom(Status::Ok, Json(user))),
        Err(_) => Err(Custom(Status::BadRequest, "Already registered".to_string())),
    }
}

/**
 * @api {delete} /emails/:email/:token Delete email
 * @apiName DeleteEmail
 * @apiGroup emails
 * @apiVersion 1.0.0
 *
 * @apiParam {String} email User's email.
 * @apiParam {String} token User's token.
 *
 * @apiSuccessExample Success-Response:
 *     HTTP/1.1 200 OK
 *     "success"
 *
 * @apiErrorExample Error-Response:
 *     HTTP/1.1 500 Internal Server Error
 *     "Not removed"
 *
 * @apiErrorExample Error-Response:
 *     HTTP/1.1 404 Not Found
 *     "Not found"
 *
 * @apiErrorExample Error-Response:
 *     HTTP/1.1 403 Forbidden
 *     "Forbidden"
 */
#[delete("/<email>/<token>")]
fn remove_user(connection: DbConn, email: String, token: String) -> Custom<String> {
    match db::delete_user(&connection, &email, &token) {
        Ok(n) if n == 0 => Custom(Status::InternalServerError, "Not removed".to_string()),
        Ok(_) => Custom(Status::Ok, "Success".to_string()),
        Err(ref s) if s == "Not found" => Custom(Status::NotFound, "Not found".to_string()),
        Err(ref s) if s == "Forbidden" => Custom(Status::Forbidden, "Forbidden".to_string()),
        Err(s) => Custom(Status::InternalServerError, format!("{}", s)),
    }
}

/**
 * @api {get} /emails/:admin_token Show all users
 * @apiDescription Show user's information only for admin.
 * @apiName GetEmails
 * @apiGroup emails
 * @apiVersion 1.0.0
 *
 * @apiParam {String} admin_token Admin token.
 *
 * @apiSuccess {Object[]} users        List of user.
 * @apiSuccess {Integer}  users.id     User's id.
 * @apiSuccess {String}   users.email  User's email.
 * @apiSuccess {String}   users.token  User's token.
 *
 * @apiSuccessExample Success-Response:
 *     HTTP/1.1 200 OK
 *     [
 *       {
 *         "id": 42,
 *         "email": "raven@os.com",
 *         "token": "azerty4242"
 *       },
 *       {...}
 *     ]
 *
 * @apiErrorExample Error-Response:
 *     HTTP/1.1 500 Internal Server Error
 *     "error"
 *
 * @apiErrorExample Error-Response:
 *     HTTP/1.1 403 Forbidden
 *     "Forbidden"
 */
#[get("/<admin_token>")]
fn get_users(
    connection: DbConn,
    server_token: AdminToken,
    admin_token: AdminToken,
) -> Result<Custom<Json<Vec<User>>>, Custom<String>> {
    if server_token != admin_token {
        return Err(Custom(Status::Forbidden, "Admin only".to_string()));
    }
    match db::get_all_users(&connection) {
        Ok(users) => Ok(Custom(Status::Ok, Json(users))),
        Err(s) => Err(Custom(Status::InternalServerError, format!("error {}", s))),
    }
}

/**
 * @api {get} /emails/:email/:admin_token Show user
 * @apiDescription Show user's information only for admin.
 * @apiName GetUser
 * @apiGroup emails
 * @apiVersion 1.0.0
 *
 * @apiParam {String} email       User's email.
 * @apiParam {String} admin_token Admin token.
 *
 * @apiSuccess {Integer} id     User's id.
 * @apiSuccess {String}  email  User's email.
 * @apiSuccess {String}  token  User's token.
 *
 * @apiSuccessExample Success-Response:
 *     HTTP/1.1 200 OK
 *     {
 *       "id": 42,
 *       "email": "raven@os.com",
 *       "token": "azerty4242"
 *     }
 *
 * @apiErrorExample Error-Response:
 *     HTTP/1.1 404 Not Found
 *     "Not found"
 *
 * @apiErrorExample Error-Response:
 *     HTTP/1.1 403 Forbidden
 *     "Admin only"
 */
#[get("/<email>/<admin_token>")]
fn get_user(
    connection: DbConn,
    server_token: AdminToken,
    email: String,
    admin_token: AdminToken,
) -> Result<Custom<Json<User>>, Custom<String>> {
    if server_token != admin_token {
        return Err(Custom(Status::Forbidden, "Admin only".to_string()));
    }
    match db::get_user(&connection, &email) {
        Ok(user) => Ok(Custom(Status::Ok, Json(user))),
        Err(s) => Err(Custom(Status::NotFound, s)),
    }
}
