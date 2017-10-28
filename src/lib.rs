#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate regex;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate uuid;
extern crate validator;
#[macro_use]
extern crate validator_derive;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

mod api;
mod error;
mod db;
mod models;
mod schemas;

use api::editions::*;
use api::ethics::*;

pub fn start() {
    dotenv::dotenv().ok();
    let pool_config = r2d2::Config::default();
    let database_url = ::std::env::var("DATABASE_URL").unwrap();
    let pool_manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> =
        r2d2::Pool::new(pool_config, pool_manager)
            .expect("Failed to create a database connection pool");

    rocket::ignite()
        .mount(
            "/",
            routes![
                edition,
                editions_index,
                editions_create,
                edition_delete,
                edition_patch,
                schema
            ],
        )
        .manage(pool)
        .launch();
}
