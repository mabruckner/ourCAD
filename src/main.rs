#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[cfg(feature = "display")]
extern crate kiss3d;
#[cfg(feature = "display")]
extern crate nalgebra;

mod boolean;
#[cfg(feature = "display")]
mod display;
mod format;
mod ops;
mod parser;
mod runtime;
mod solid;

use ops::*;
use solid::*;

use std::fs::File;

fn test_boolean() {
  let outside_box = Solid::make_box([2.0, 2.0, 2.0]);
  let test_plane = Plane {
    point: Point {
      pos: [0.0, 0.0, 0.0].into(),
    },
    norm: Vector::from([1.0, 1.0, 1.0]).into(),
  };
  println!("{:?}", slice(&outside_box, &test_plane));
  let inside_box = Transform::rotate_x(1.0) * Solid::make_box([1.0, 5.0, 1.0]);
  let bool_result = boolean(&outside_box, &inside_box, Boolean::Difference);
  println!("{:?}", bool_result);
  #[cfg(feature = "display")]
  {
    /*let mut display = display::KissDisplay::new();
    display.set(inside_box);
    display.join();*/
    let tris = triangulate_solid(bool_result.clone());
    display::quick_display(tris.iter().map(tri_to_face).collect());
    display::display(bool_result.clone());
    format::write_stl(
      &mut File::create("output.stl").unwrap(),
      bool_result,
      "test output",
    )
    .unwrap();
  }
}

fn main() {
  if let Some(ast) = parser::parse::parse() {
    runtime::Runtime::new().run(&ast).unwrap();
  }
  #[cfg(feature = "display")]
  test_boolean();
  // println!("{:?}", Solid::make_box([2.0, 2.0, 2.0]));
}
