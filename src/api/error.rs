use diesel::result::Error as DieselError;
use r2d2::GetTimeout;
use uuid;
use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;

error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }

    foreign_links {
        Db(DieselError);
        DbTimeout(GetTimeout);
        Json(::json::Error);
        UuidParseError(uuid::ParseError);
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _req: &Request) -> Result<Response<'r>, Status> {
        match *self.kind() {
            ErrorKind::Db(DieselError::NotFound) => Err(Status::NotFound),
            ErrorKind::UuidParseError(_) => Err(Status::BadRequest),
            _ => Err(Status::InternalServerError),
        }
    }
}
