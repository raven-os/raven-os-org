//! Contains all user routes

use config::{AdminToken, DbConn};
use db::{self, models::User};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::Route;
use rocket_contrib::Json;

/// Returns all user routes
///
/// TODO: temporary disabled get routes not to collide with front
pub fn get_routes() -> Vec<Route> {
    routes![create, remove]
}

/**
 * @api {post} /emails/:email Add email
 * @apiName creates
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
fn create(connection: DbConn, email: String) -> Result<Custom<Json<User>>, Custom<String>> {
    match db::create_user(&connection, &email) {
        Ok(user) => Ok(Custom(Status::Ok, Json(user))),
        Err(_) => Err(Custom(Status::BadRequest, "Already registered".to_string())),
    }
}

/**
 * @api {delete} /emails/:email/:token Delete email
 * @apiName remove
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
fn remove(connection: DbConn, email: String, token: String) -> Custom<String> {
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
 * @apiName get_all
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
fn get_all(
    connection: DbConn,
    server_token: AdminToken,
    admin_token: AdminToken,
) -> Result<Custom<Json<Vec<User>>>, Custom<String>> {
    if server_token != admin_token {
        Err(Custom(Status::Forbidden, "Admin only".to_string()))
    } else {
        match db::get_all_users(&connection) {
            Ok(users) => Ok(Custom(Status::Ok, Json(users))),
            Err(s) => Err(Custom(Status::InternalServerError, format!("error: {}", s))),
        }
    }
}

/**
 * @api {get} /emails/:email/:admin_token Show user
 * @apiDescription Show user's information only for admin.
 * @apiName get
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
fn get(
    connection: DbConn,
    server_token: AdminToken,
    email: String,
    admin_token: AdminToken,
) -> Result<Custom<Json<User>>, Custom<String>> {
    if server_token != admin_token {
        Err(Custom(Status::Forbidden, "Admin only".to_string()))
    } else {
        match db::get_user(&connection, &email) {
            Ok(user) => Ok(Custom(Status::Ok, Json(user))),
            Err(s) => Err(Custom(Status::NotFound, s)),
        }
    }
}
