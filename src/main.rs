#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate diesel;
extern crate handlebars;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

use rocket_contrib::Template;

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

#[get("/")]
fn index() -> Template {
    Template::render("index", ParsContainer {
        pars: vec!["pars 1", "pars 2", "pars 3", "pars 4"].into_iter().map(m).collect()
    })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files])
        .attach(Template::fairing())
        .launch();
}
