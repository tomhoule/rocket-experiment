extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate purescript_waterslide;
#[macro_use] extern crate purescript_waterslide_derive;

mod schemas;

use purescript_waterslide::*;
use schemas::ethica::*;

fn main() {
    let module = purs_module!("Schema".to_string() ; Schema, Node, NumberedFragment, ScopeDescriptor);
    print!("{}", module)
}
