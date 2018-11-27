//! Contains all newsletter routes

use crate::json;
use rocket::http::Status;
use rocket::State;
use rocket_contrib::Json;

use crate::app::newsletter::Newsletter;
use crate::app::{ApiError, ApiResult};
use crate::db::newsletter::NewsletterUser;
use crate::db::DbConnection;

// The following structures are used as parameter for API endpoints

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
struct NewUser {
    email: String,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
struct IdentifiedUser {
    email: String,
    token: String,
}

/**
 * @api {POST} /newsletter/ Adds an email to the newsletter
 * @apiName NewsletterAddEmail
 * @apiGroup Newsletter
 * @apiVersion 1.0.0
 *
 * @apiParam {String}   email   User's email address
 *
 * @apiParamExample {json} Request-Example:
 *      {
 *        "email": "john-doe@example.com"
 *      }
 *
 * @apiSuccess {Integer} id     User's id.
 * @apiSuccess {String}  email  User's email.
 * @apiSuccess {String}  token  User's token.
 *
 * @apiSuccessExample Response (Success):
 *      HTTP/1.1 201 Created
 *      {
 *        "id": 42,
 *        "email": "john-doe@example.com",
 *        "token": "414c1754-b274-4db9-9b92-9dc7a8aca7fc"
 *      }
 *
 * @apiErrorExample Response (Error):
 *       HTTP/1.1 400 Bad Request
 *       {
 *         "error": "already_registered",
 *         "error_description":"the user is already registered"
 *       }
 */
#[post("/", format = "application/json", data = "<data>")]
fn add(
    newsletter: State<Newsletter>,
    connection: DbConnection,
    data: Json<NewUser>,
) -> ApiResult<NewsletterUser, ApiError> {
    if let Ok(user) = newsletter.add_user(&connection, &data.email) {
        ApiResult::success(Status::Created, user)
    } else {
        ApiResult::error(
            Status::BadRequest,
            ApiError::from("already_registered", "the user is already registered"),
        )
    }
}

/**
 * @api {DELETE} /newsletter/ Removes an email of the newsletter
 * @apiName NewsletterRemoveEmail
 * @apiGroup Newsletter
 * @apiVersion 1.0.0
 *
 * @apiParam {String}   email     User's email.
 * @apiParam {String}   token     User's token.
 *
 * @apiSuccessExample Response (Success):
 *      HTTP/1.1 200 OK
 *      {}
 *
 * @apiErrorExample Response (Email not found):
 *      HTTP/1.1 404 Not Found
 *      {
 *        "error": "not_found",
 *        "error_description": "the email isn't registered in the newsletter"
 *      }
 *
 * @apiErrorExample Response (Invalid token):
 *      HTTP/1.1 403 Forbidden
 *      {
 *        "error": "forbidden",
 *        "error_description": "you are not authorized to remove this email"
 *      }
 */
#[delete("/", format = "application/json", data = "<data>")]
fn remove(
    newsletter: State<Newsletter>,
    connection: DbConnection,
    data: Json<IdentifiedUser>,
) -> ApiResult<json::Value, ApiError> {
    if let Err((code, error)) = newsletter.remove_user(&connection, &data.email, &data.token) {
        ApiResult::error(code, error)
    } else {
        ApiResult::success(Status::Ok, json::Value::Object(json::Map::new()))
    }
}

/**
 * @api {GET} /:admin_token Dumps all users registered to the newsletter.
 * @apiName NewsletterDumpUsers
 * @apiGroup Newsletter
 * @apiVersion 1.0.0
 *
 * @apiParam {String}       admin_token  Admin token.
 *
 * @apiSuccess {Object[]}   users        List of user.
 * @apiSuccess {Integer}    users.id     User's id.
 * @apiSuccess {String}     users.email  User's email.
 * @apiSuccess {String}     users.token  User's token.
 *
 * @apiSuccessExample Success-Response:
 *      HTTP/1.1 200 OK
 *      [
 *        {
 *          "id": 42,
 *          "email": "john-doe@example.com",
 *          "token": "414c1754-b274-4db9-9b92-9dc7a8aca7fc"
 *        },
 *        {...}
 *      ]
 *
 * @apiErrorExample Error-Response:
 *      HTTP/1.1 403 Forbidden
 *      {
 *        "error": "forbidden",
 *        "error_description": "admin token is invalid"
 *      }
 */
#[get("/<admin_token>")]
fn dump(
    newsletter: State<Newsletter>,
    connection: DbConnection,
    admin_token: String,
) -> ApiResult<Vec<NewsletterUser>, ApiError> {
    if newsletter.admin_token() == admin_token {
        if let Ok(users) = newsletter.users(&connection) {
            ApiResult::success(Status::Ok, users)
        } else {
            ApiResult::error(Status::InternalServerError, ApiError::internal_error())
        }
    } else {
        ApiResult::error(
            Status::Forbidden,
            ApiError::from("forbidden", "admin token is invalid"),
        )
    }
}
