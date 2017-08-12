extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate purescript_waterslide;
#[macro_use]
extern crate purescript_waterslide_derive;
extern crate rocket;
extern crate uuid;

mod db;
mod models;
mod schemas;

use purescript_waterslide::*;
use schemas::ethica::*;
use models::edition::*;

fn main() {
    let module = purs_module!(
        "Models".to_string() ;
        Schema,
        Node,
        NumberedFragment,
        ScopeDescriptor,
        Edition,
        EditionNew);
    print!("{}", module)
}
