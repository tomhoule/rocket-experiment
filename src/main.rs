#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate error_chain;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_urlencoded;
extern crate uuid;

use rocket::request::{Form};
use rocket::response::{Flash, Redirect, NamedFile, Responder};
use rocket_contrib::Json;
use std::path::{Path, PathBuf};
use schemas::ethica::Schema;

use std::collections::BTreeMap;

mod db;
mod models;
mod schemas;

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("css/").join(file)).ok()
}

#[post("/api/editions", data="<edition>")]
fn editions_create(edition: Json<models::EditionNew>) -> Json<String> {
    unimplemented!();
}

#[get("/api/editions")]
fn editions_index() -> Json<String> {
    unimplemented!();
}

#[get("/api/editions/<id>")]
fn edition(id: i32) -> Json<String> {
    unimplemented!();
}

#[get("/api/schemas/ethica")]
fn schemas_ethica() -> Json<Schema> {
    unimplemented!();
}

fn main() {
    rocket::ignite()
        .mount("/", routes![edition, editions_index, editions_create, schemas_ethica])
        .launch();
}
