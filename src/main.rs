#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod compiler;
mod solid;

use solid::*;

fn main() {
    // println!("{:?}", Solid::make_box([2.0, 2.0, 2.0]));
    compiler::compile::compile();
}
