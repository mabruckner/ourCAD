#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[cfg(feature="display")]
extern crate kiss3d;
#[cfg(feature="display")]
extern crate nalgebra;

mod compiler;
mod solid;
mod boolean;
#[cfg(feature="display")]
mod display;

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
  let bool_result = boolean(&outside_box, &inside_box, Boolean::Difference);
  println!("{:?}", bool_result);
  #[cfg(feature="display")]
  {
    /*let mut display = display::KissDisplay::new();
    display.set(inside_box);
    display.join();*/
    display::display(bool_result);
  }
}

fn main() {
  compiler::compile::compile();
  #[cfg(feature="display")]
  test_boolean();
}
