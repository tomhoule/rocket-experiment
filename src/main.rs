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
extern crate purescript_waterslide;
#[macro_use]
extern crate purescript_waterslide_derive;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate uuid;

use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

mod api;
mod db;
mod models;
mod pages;
mod schemas;

use api::editions::*;
use api::ethica::*;
use pages::*;

#[get("/static/css/<file..>")]
fn css(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("css/").join(file)).ok()
}

#[get("/static/js/<file..>")]
fn js(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("js/dist/").join(file)).ok()
}


fn main() {
    dotenv::dotenv().ok();
    let pool_config = r2d2::Config::default();
    let database_url = ::std::env::var("DATABASE_URL").unwrap();
    let pool_manager =
        ConnectionManager::<PgConnection>::new(database_url);
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> =
        r2d2::Pool::new(pool_config, pool_manager)
            .expect("Failed to create a database connection pool");

    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                ethica_index,
                ethica_new,
                edition,
                editions_index,
                editions_create,
                edition_delete,
                edition_patch,
                css,
                js,
                schema
            ],
        )
        .attach(rocket_contrib::Template::fairing())
        .manage(pool)
        .launch();
}
