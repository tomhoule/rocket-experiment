use diesel::result::Error as DieselError;
use r2d2::GetTimeout;
use validator;
use grpcio::{RpcStatus, RpcStatusCode};
use json::{to_value, Value};
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
    }

    errors {
        Validation(errs: validator::ValidationErrors) {
            description("Invalid input from client")
            display("Invalid message: {:?}", errs)
        }
    }
}

fn validation_errors_to_json(errs: validator::ValidationErrors) -> Value {
    use std::collections::HashMap;
    let map: HashMap<&str, String> = errs.inner()
        .into_iter()
        .map(|(k, v)| {
            let joined: String = v.into_iter().fold(String::new(), |mut acc, err| {
                if let Some(msg) = err.message {
                    acc.push_str(&msg);
                    acc.push_str("\n");
                }
                acc
            });
            (k, joined)
        })
        .collect();
    to_value(&map).unwrap()
}

fn report<T: ::std::fmt::Display>(status: RpcStatusCode, err: T) -> RpcStatus {
    RpcStatus {
        status,
        details: Some(format!("{}", err)),
    }
}

impl Error {
    pub fn into_grpc_status(self) -> RpcStatus {
        use self::ErrorKind::*;
        match self {
            Error(Validation(errs), _) => report(
                RpcStatusCode::InvalidArgument,
                validation_errors_to_json(errs),
            ),
            other => report(RpcStatusCode::Internal, format!("{}", other)),
        }
    }
}

impl ::std::convert::From<validator::ValidationErrors> for Error {
    fn from(errs: validator::ValidationErrors) -> Error {
        ErrorKind::Validation(errs).into()
    }
}
