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

use diesel::mysql::MysqlConnection;
use r2d2_diesel::ConnectionManager;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use rocket_contrib::Json;

use rocket::response::status::Custom;

mod db;

use db::models::User;

fn main() {
    dotenv::dotenv().ok();

    let database_url = match std::env::var("DATABASE_URL") {
        Ok(s) => s,
        Err(_) => {
            eprintln!("error: the DATABASE_URL variable is not set.");
            process::exit(1);
        }
    };

    rocket::ignite()
        .manage(init_pool(&database_url))
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
 *     HTTP/1.1 500 Bad Request
 *     "Not removed"
 *
 * @apiErrorExample Error-Response:
 *     HTTP/1.1 404 Not Found
 *     "Not found"
 *
 * @apiErrorExample Error-Response:
 *     HTTP/1.1 500 Internal Server Error
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
 * @api {get} /emails Show all users
 * @apiName GetEmails
 * @apiGroup emails
 * @apiVersion 1.0.0
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
 */
#[get("/")]
fn get_users(connection: DbConn) -> Result<Custom<Json<Vec<User>>>, Custom<String>> {
    match db::get_all_users(&connection) {
        Ok(users) => Ok(Custom(Status::Ok, Json(users))),
        Err(s) => Err(Custom(Status::InternalServerError, format!("error {}", s))),
    }
}

/**
 * @api {get} /emails/:email Show user
 * @apiName GetUser
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
 *     HTTP/1.1 404 Not Found
 *     "Not found"
 */
#[get("/<email>")]
fn get_user(connection: DbConn, email: String) -> Result<Custom<Json<User>>, Custom<String>> {
    match db::get_user(&connection, &email) {
        Ok(user) => Ok(Custom(Status::Ok, Json(user))),
        Err(s) => Err(Custom(Status::NotFound, s)),
    }
}

/* ==================== DATABASE CONNECTION WRAPPER ==================== */

/// Initializes a database pool.
fn init_pool(database_url: &str) -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("error creating database pool")
}

///Connection Guard
use std::ops::Deref;

/// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &DbConn as an &MysqlConnection.
impl Deref for DbConn {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
