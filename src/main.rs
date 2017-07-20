#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate diesel;
#[macro_use]
extern crate error_chain;
extern crate handlebars;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_urlencoded;
extern crate uuid;

use rocket::request::{Form};
use rocket::response::{Flash, Redirect, NamedFile, Responder};
use rocket::response::content::Json;
use std::path::{Path, PathBuf};

use rocket_contrib::Template;
use std::collections::BTreeMap;

mod models;
mod schemas;

#[derive(Serialize)]
struct MessageContainer {
    message: &'static str,
}

fn m(s: &'static str) -> MessageContainer {
    MessageContainer { message: s }
}

#[derive(Serialize)]
struct ParsContainer {
    pars: Vec<MessageContainer>,
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("css/").join(file)).ok()
}

#[get("/ethica")]
fn ethica_index() -> Template {
    Template::render("index", ParsContainer {
        pars: vec!["pars 1", "pars 2", "pars 3", "pars 4"].into_iter().map(m).collect()
    })
}

#[get("/intern")]
fn intern_index() -> Template {
    let mut context = BTreeMap::new();
    context.insert("schema", schemas::ethica::ETHICA);
    Template::render("intern", context)
}

#[post("/intern/editions/create", data="<edition>")]
fn editions_create(edition: models::EditionNew) -> Json<String> {
    unimplemented!();
}

fn main() {
    rocket::ignite()
        .mount("/", routes![ethica_index, intern_index, files])
        .attach(Template::fairing())
        .launch();
}
