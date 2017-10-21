use diesel::result::Error as DieselError;
use r2d2::GetTimeout;
use grpcio::{
    RpcStatus,
    RpcStatusCode
};
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
}

impl Error {
    pub fn into_grpc_status(self) -> RpcStatus {
        RpcStatus {
            status: RpcStatusCode::Unimplemented,
            details: None,
        }
    }
}
