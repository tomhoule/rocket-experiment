use diesel::result::Error as DieselError;
use r2d2::GetTimeout;
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
