use failure::Error;
use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;

use diesel::result::Error as DieselError;
use r2d2::GetTimeout;

#[derive(Debug, Fail)]
pub enum Failure {
    #[fail(display = "Database error: {}", _0)] Db(#[cause] DieselError),
    #[fail(display = "Database conn timeout: {}", _0)] DbTimeout(#[cause] GetTimeout),
    #[fail(display = "Server error: {}", _0)] ServerError(Error),
}

macro_rules! impl_from {
    ($host:ty, $target:ty, $construct:expr) => {
        impl From<$target> for $host {
            fn from(target: $target) -> $host {
                $construct(target.into())
            }
        }
    }
}

impl_from!(Failure, DieselError, Failure::Db);
impl_from!(Failure, GetTimeout, Failure::DbTimeout);
impl_from!(Failure, ::json::Error, Failure::ServerError);

impl<'r> Responder<'r> for Failure {
    fn respond_to(self, _req: &Request) -> Result<Response<'r>, Status> {
        use self::Failure::*;
        match self {
            Db(DieselError::NotFound) => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        }
    }
}