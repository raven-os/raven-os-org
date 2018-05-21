//! Database business layer
//!
//! Contains actions to interact with a database using `Diesel`

pub mod newsletter;

use rocket::http::Status;
use rocket::request::{self, FromRequest, State};
use rocket::{Outcome, Request};

use diesel::mysql::MysqlConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;

use app::App;

/// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConnection(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

// DbConnection is only a wrapper around a MysqlConnection
impl AsRef<MysqlConnection> for DbConnection {
    fn as_ref(&self) -> &MysqlConnection {
        &self.0
    }
}

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConnection, ()> {
        let app = request.guard::<State<App>>()?;
        match app.pool().get() {
            Ok(conn) => Outcome::Success(DbConnection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
