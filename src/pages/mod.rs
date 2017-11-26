mod ethics;
mod fail;

pub use self::ethics::*;

use diesel::pg::PgConnection;
use fluent::MessageContext;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use rocket::State;

type I18n<'a> = State<'a, MessageContext<'static>>;

type DbConn<'a> = State<'a, Pool<ConnectionManager<PgConnection>>>;
