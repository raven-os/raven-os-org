//! All the configurations for the application
//!
//! Contains types definitions and traits implementations
//! for managed states used with `.manage()` on a rocket app`

use diesel::mysql::MysqlConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::RawStr;
use rocket::http::Status;
use rocket::request::{self, FromParam, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

/// Encapsulate a r2d2 Mysql connection pool.
type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

/// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

/// A string representing admin token to access admin only endpoints
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct AdminToken(pub String);

/// Contains all the application configuration objects
pub struct MyConfig {
    /// The database pool
    pub pool: Pool,
    /// The admin token
    pub admin_token: AdminToken,
}

/// Initializes a database pool.
pub fn init_pool(database_url: &str) -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("error: can't create database pool")
}

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let config = request.guard::<State<MyConfig>>()?;
        match config.pool.get() {
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

/// Retrieves the managed admin token
impl<'a, 'r> FromRequest<'a, 'r> for AdminToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminToken, ()> {
        request
            .guard::<State<MyConfig>>()
            .map(|config| config.admin_token.clone())
    }
}

/// Convert the dynamic segment admin_token into AdminToken type
impl<'a> FromParam<'a> for AdminToken {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<AdminToken, Self::Error> {
        Ok(AdminToken(param.to_string()))
    }
}
