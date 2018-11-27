use rocket::http::Status;

use diesel;
use diesel::prelude::*;

use failure::Error;
use uuid::Uuid;

use crate::app::ApiError;
use crate::db::newsletter::models::{NewNewsletterUser, NewsletterUser};
use crate::db::newsletter::schema::newsletter_users;
use crate::db::DbConnection;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Newsletter {
    admin_token: String,
}

impl Newsletter {
    #[inline]
    pub fn new(admin_token: String) -> Newsletter {
        Newsletter { admin_token }
    }

    #[inline]
    pub fn admin_token(&self) -> &str {
        &self.admin_token
    }

    #[inline]
    pub fn generate_token(&self) -> String {
        Uuid::new_v4().hyphenated().to_string()
    }

    pub fn add_user(&self, db_con: &DbConnection, email: &str) -> Result<NewsletterUser, Error> {
        let token = &self.generate_token();

        let new_user = NewNewsletterUser { email, token };

        diesel::insert_into(newsletter_users::table)
            .values(&new_user)
            .execute(db_con.as_ref())?;
        Ok(newsletter_users::table
            .order(newsletter_users::id.desc())
            .first(db_con.as_ref())?)
    }

    pub fn remove_user(
        &self,
        db_con: &DbConnection,
        uemail: &str,
        utoken: &str,
    ) -> Result<(), (Status, ApiError)> {
        use crate::db::newsletter::schema::newsletter_users::dsl::*;

        let user = newsletter_users
            .filter(email.eq(uemail))
            .first::<NewsletterUser>(db_con.as_ref())
            .map_err(|_| {
                (
                    Status::NotFound,
                    ApiError::from("not_found", "the email isn't registered in the newsletter"),
                )
            })?;
        if user.token() == utoken {
            if diesel::delete(newsletter_users.filter(email.eq(user.email())))
                .execute(db_con.as_ref())
                .is_ok()
            {
                Ok(())
            } else {
                Err((Status::InternalServerError, ApiError::internal_error()))
            }
        } else {
            Err((
                Status::Forbidden,
                ApiError::from("forbidden", "you are not authorized to remove this email"),
            ))
        }
    }

    pub fn users(&self, db_con: &DbConnection) -> Result<Vec<NewsletterUser>, Error> {
        use crate::db::newsletter::schema::newsletter_users::dsl::*;

        Ok(newsletter_users.load::<NewsletterUser>(db_con.as_ref())?)
    }
}
