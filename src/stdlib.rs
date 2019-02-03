use runtime::Object;

pub fn print(args: Vec<Object>) -> Option<Object> {
  if let Some(arg) = args.get(0) {
    println!("{}", arg);
  } else {
    println!("Error: no arg");
  }
  None
}
