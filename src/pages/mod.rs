mod ethics;

pub use self::ethics::*;

use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use rocket::State;

type DbConn<'a> = State<'a, Pool<ConnectionManager<PgConnection>>>;
