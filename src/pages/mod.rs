mod ethics;
mod ethics_tests;
mod fail;

pub use self::ethics::*;

use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use rocket::State;
use i18n::I18nContexts;

type I18n<'a> = State<'a, I18nContexts>;

type DbConn<'a> = State<'a, Pool<ConnectionManager<PgConnection>>>;
