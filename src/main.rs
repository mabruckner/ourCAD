#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod compiler;
mod solid;
mod boolean;

use solid::*;
use boolean::*;

fn test_boolean() {
  let outside_box = Solid::make_box([2.0, 2.0, 2.0]);
  let test_plane = Plane {
    point: Point { pos: [0.0, 0.0, 0.0].into() },
    norm: Vector::from([1.0, 1.0, 1.0]).into(),
  };
  println!("{:?}", slice(&outside_box, &test_plane));
  let inside_box = Solid::make_box([1.0, 5.0, 1.0]);
  println!("{:?}",
           boolean(&outside_box, &inside_box, Boolean::Difference));
}

fn main() {
  // println!("{:?}", Solid::make_box([2.0, 2.0, 2.0]));
  compiler::compile::compile();
}
