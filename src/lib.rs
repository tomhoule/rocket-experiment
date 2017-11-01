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
extern crate futures;
extern crate grpcio;
#[macro_use]
extern crate lazy_static;
extern crate protobuf;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate regex;
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json as json;
extern crate uuid;
extern crate validator;
#[macro_use]
extern crate validator_derive;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

mod api;
pub mod rpc_api;
mod error;
mod files;
pub mod db;
pub mod models;
mod pages;
pub mod rpc;
mod schemas;

use api::editions::*;
use api::ethics::*;

use rocket_contrib::Template;

#[get("/")]
fn index() -> Template {
    let context = json!({});
    Template::render("index", &context)
}

pub fn start() {
    dotenv::dotenv().ok();
    let pool_config = r2d2::Config::default();
    let database_url = ::std::env::var("DATABASE_URL").unwrap();
    let pool_manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> =
        r2d2::Pool::new(pool_config, pool_manager)
            .expect("Failed to create a database connection pool");

    let cors_options: rocket_cors::Cors = ::std::default::Default::default();

    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                files::files,
                pages::editions_index,
                pages::editions_create,
                pages::editions_new,
                edition,
                editions_index,
                editions_create,
                edition_delete,
                edition_patch,
                schema,
            ],
        )
        .attach(cors_options)
        .attach(Template::fairing())
        .manage(pool)
        .launch();
}
