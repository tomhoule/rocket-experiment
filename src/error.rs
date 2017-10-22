use diesel::result::Error as DieselError;
use r2d2::GetTimeout;
use validator;
use grpcio::{RpcStatus, RpcStatusCode};
use uuid;

error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }

    foreign_links {
        Db(DieselError);
        DbTimeout(GetTimeout);
        Json(::json::Error);
        UuidParseError(uuid::ParseError);
        ValidationError(validator::ValidationError);
    }

    errors {
        Validation(errs: validator::ValidationErrors) {
            description("Invalid input from client")
            display("Invalid message: {:?}", errs)
        }
    }
}

impl Error {
    pub fn into_grpc_status(self) -> RpcStatus {
        RpcStatus {
            status: RpcStatusCode::Unimplemented,
            details: None,
        }
    }
}

impl ::std::convert::From<validator::ValidationErrors> for Error {
    fn from(errs: validator::ValidationErrors) -> Error {
        ErrorKind::Validation(errs).into()
    }
}
