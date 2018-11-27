pub mod newsletter;

use rocket::http::Status;
use rocket::response::Responder;
use rocket::response::{self, content};
use rocket::Request;

use diesel::mysql::MysqlConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

use crate::json;
use failure::Error;
use serde::Serialize;

pub struct App {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl App {
    #[inline]
    pub fn from(database_url: &str) -> Result<App, Error> {
        Ok(App {
            pool: Pool::new(ConnectionManager::<MysqlConnection>::new(database_url))?,
        })
    }

    pub fn pool(&self) -> &Pool<ConnectionManager<MysqlConnection>> {
        &self.pool
    }
}

// Common types used by the whole API

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ApiResult<T: Serialize, E: Serialize> {
    code: Status,
    content: Result<T, E>,
}

impl<T: Serialize, E: Serialize> ApiResult<T, E> {
    pub fn success(code: Status, content: T) -> ApiResult<T, E> {
        ApiResult {
            code,
            content: Ok(content),
        }
    }

    pub fn error(code: Status, content: E) -> ApiResult<T, E> {
        ApiResult {
            code,
            content: Err(content),
        }
    }
}

impl<T: Serialize, E: Serialize> Responder<'static> for ApiResult<T, E> {
    fn respond_to(self, req: &Request) -> response::Result<'static> {
        let res = {
            match &self.content {
                Ok(t) => json::to_string(t),
                Err(e) => json::to_string(e),
            }
        };
        res.map(|mut string| {
            string.push('\n');
            let mut req = content::Json(string).respond_to(req).unwrap();
            req.set_status(self.code);
            req
        })
        .map_err(|_| Status::InternalServerError)
    }
}

/// A unified error structure to return for API endpoints that failed
/// This makes our request predictable because they all fail the same way.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ApiError {
    error: String,
    error_description: String,
}

impl ApiError {
    pub fn from(error: &str, error_description: &str) -> ApiError {
        ApiError {
            error: String::from(error),
            error_description: String::from(error_description),
        }
    }

    pub fn internal_error() -> ApiError {
        ApiError::from("internal_error", "the operation failed")
    }
}
