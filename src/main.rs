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
use rocket::response::{Flash, Redirect, NamedFile};
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

#[get("/")]
fn index() -> Template {
    Template::render("index", ParsContainer {
        pars: vec!["pars 1", "pars 2", "pars 3", "pars 4"].into_iter().map(m).collect()
    })
}

#[get("/ethica/editions/new")]
fn editions_new() -> Template {
    let mut context = BTreeMap::new();
    context.insert("schema", schemas::ethica::ETHICA);
    Template::render("ethica/editions/new", context)
}

#[post("/ethica/editions/create", data="<edition>")]
fn editions_create(edition: Result<models::EditionNew, models::ValidationError>) -> Flash<Redirect> {
    match edition {
        Ok(edition) => {
            println!("Parsed an edition! {:?}", edition);
            unimplemented!()
        },
        Err(models::ValidationError(models::ValidationErrorKind::Serde(edition), _)) => {
            println!("Invalid form -> {:?}", edition);
            unimplemented!()
        },
        _ => panic!(),
    }
    println!("{:?}", edition);
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, editions_new, editions_create, files])
        .attach(Template::fairing())
        .launch();
}
