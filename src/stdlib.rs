use boolean::{boolean, Boolean};
#[cfg(feature = "display")]
use display::display;
use format::write_stl;
use runtime::{get_number, get_solid, get_str, Object, RuntimeError};
use solid::{Plane, Point, Solid, Transform, Vector};
use std::fs::File;

pub fn std_print(args: Vec<Object>) -> Result<Object, RuntimeError> {
  if let Some(arg) = args.get(0) {
    println!("{:?}", arg);
  } else {
    println!("Error: no arg");
  }
  Ok(Object::Number(0.0))
}

pub fn std_make_box(args: Vec<Object>) -> Result<Object, RuntimeError> {
  let l = get_number(args.get(0).unwrap().clone())?;
  let w = get_number(args.get(1).unwrap().clone())?;
  let h = get_number(args.get(2).unwrap().clone())?;
  Ok(Object::Solid(Solid::make_box([l, w, h])))
}

pub fn std_make_plane(args: Vec<Object>) -> Result<Object, RuntimeError> {
  let p1 = get_number(args.get(0).unwrap().clone())?;
  let p2 = get_number(args.get(1).unwrap().clone())?;
  let p3 = get_number(args.get(2).unwrap().clone())?;
  let plane = Plane {
    point: Point {
      pos: [p1, p2, p3].into(),
    },
    norm: Vector::from([1.0, 1.0, 1.0]).into(),
  };
  Ok(Object::Plane(plane))
}

pub fn std_move(args: Vec<Object>) -> Result<Object, RuntimeError> {
  Ok(Object::Number(0.0))
}

pub fn std_difference(args: Vec<Object>) -> Result<Object, RuntimeError> {
  let box1 = get_solid(args.get(0).unwrap().clone())?;
  let box2 = get_solid(args.get(1).unwrap().clone())?;
  let diff = boolean(&box1, &box2, Boolean::Difference);
  Ok(Object::Solid(diff))
}

pub fn std_rotate_x(args: Vec<Object>) -> Result<Object, RuntimeError> {
  let box1 = get_solid(args.get(0).unwrap().clone())?;
  let angle = get_number(args.get(1).unwrap().clone())?;
  let rotated = Transform::rotate_x(angle) * box1;
  Ok(Object::Solid(rotated))
}

pub fn std_display(args: Vec<Object>) -> Result<Object, RuntimeError> {
  let solid = get_solid(args.get(0).unwrap().clone())?;
  #[cfg(feature = "display")]
  display(solid);
  Ok(Object::Number(0.0))
}

pub fn std_write_stl(args: Vec<Object>) -> Result<Object, RuntimeError> {
  let solid = get_solid(args.get(0).unwrap().clone())?;
  let name = get_str(args.get(1).unwrap().clone())?;
  write_stl(&mut File::create(&name).unwrap(), solid, "test output").unwrap();
  Ok(Object::Number(0.0))
}
