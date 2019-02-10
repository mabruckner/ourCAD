use runtime::{Object, RuntimeError};

pub fn std_print(args: Vec<Object>) -> Result<Object, RuntimeError> {
  if let Some(arg) = args.get(0) {
    println!("{:?}", arg);
  } else {
    println!("Error: no arg");
  }
  Ok(Object::Number(0.0))
}

pub fn std_box(args: Vec<Object>) -> Result<Object, RuntimeError> {
  Ok(Object::Number(0.0))
}

pub fn std_move(args: Vec<Object>) -> Result<Object, RuntimeError> {
  Ok(Object::Number(0.0))
}

pub fn std_difference(args: Vec<Object>) -> Result<Object, RuntimeError> {
  Ok(Object::Number(0.0))
}
