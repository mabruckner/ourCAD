use parser::ast::{Expr, Meta, Operator, Stmt};
use parser::util::get_line_number;
use solid::{Edge, Face, Plane, Point, Solid, Vector};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use stdlib;

const CURRENT_FUNCTION_CALL_KEY: &'static str = "___CURRENT_FUNCTION_CALL";

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
  Number(f64),
  Point(Point),
  Edge(Edge),
  Plane(Plane),
  Face(Face),
  Vector(Vector),
  Solid(Solid),
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
  msg: String,
}

impl RuntimeError {
  fn new(msg: String) -> RuntimeError {
    RuntimeError { msg: msg }
  }
}

impl fmt::Display for RuntimeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl Error for RuntimeError {
  fn description(&self) -> &str {
    self.msg.as_str()
  }

  fn cause(&self) -> Option<&Error> {
    None
  }
}

#[derive(Debug, Clone, PartialEq)]
enum SymbolVal {
  Function(Vec<String>, Meta<Stmt>),
  StdLib(String),
  // Object(Box<Fn(dyn HashMap<String, Object>) -> Object>),
  Object(Object),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolEntry {
  name: String,
  value: SymbolVal,
}

pub struct Runtime {
  symbol_table: Vec<HashMap<String, SymbolEntry>>,
  std_lib_functions: Vec<&'static str>,
  source_code: String,
}

impl Runtime {
  pub fn new(source_code: String) -> Runtime {
    Runtime {
      symbol_table: vec![HashMap::new()],
      std_lib_functions: vec![
        "print",
        "Box",
        "Plane",
        "move",
        "difference",
        "rotate_x",
        "display",
      ],
      source_code: source_code,
    }
  }

  pub fn run(&mut self, program: &Vec<Meta<Stmt>>) -> Result<(), RuntimeError> {
    self.add_stdlib();
    for stmt in program {
      self.run_stmt(stmt)?;
    }
    Ok(())
  }

  fn run_stmt(&mut self, stmt: &Meta<Stmt>) -> Result<(), RuntimeError> {
    match stmt.inside {
      Stmt::Block(ref stmts) => self.handle_block(stmts),
      Stmt::Return(ref expr) => self.handle_return(expr),
      Stmt::Expr(ref expr) => self.handle_expr(expr),
      Stmt::Assign(ref identifier, ref expr) => self.handle_assign(identifier.to_string(), expr),
      Stmt::Function(ref identifier, ref params, ref stmt) => {
        self.handle_function(identifier.to_string(), params, stmt)
      }
      Stmt::For(ref assign, ref condition, ref inc, ref body) => {
        self.handle_for(assign, condition, inc, body)
      }
      Stmt::If(ref condition, ref body) => self.handle_if(condition, body),
    }
  }

  fn handle_for(
    &mut self,
    assign: &Meta<Stmt>,
    cond: &Meta<Expr>,
    inc: &Meta<Stmt>,
    body: &Meta<Stmt>,
  ) -> Result<(), RuntimeError> {
    self.symbol_table.push(HashMap::new());
    self.run_stmt(assign)?;
    while get_number(self.run_expr(cond)?)? > 0.0 {
      self.symbol_table.push(HashMap::new());
      self.run_stmt(body)?;
      self.symbol_table.pop();
      self.run_stmt(inc)?;
    }
    self.symbol_table.pop();
    Ok(())
  }

  fn handle_if(&mut self, cond: &Meta<Expr>, body: &Meta<Stmt>) -> Result<(), RuntimeError> {
    if get_number(self.run_expr(cond)?)? > 0.0 {
      self.symbol_table.push(HashMap::new());
      self.run_stmt(body)?;
      self.symbol_table.pop();
    }
    Ok(())
  }

  fn handle_block(&mut self, stmts: &Vec<Meta<Stmt>>) -> Result<(), RuntimeError> {
    self.symbol_table.push(HashMap::new());
    for stmt in stmts {
      self.run_stmt(&stmt)?;
    }
    self.symbol_table.pop();
    Ok(())
  }

  fn handle_return(&mut self, expr: &Meta<Expr>) -> Result<(), RuntimeError> {
    println!("Returning: {:?}", self.run_expr(&expr)?);
    Ok(())
  }

  fn handle_expr(&mut self, expr: &Meta<Expr>) -> Result<(), RuntimeError> {
    self.run_expr(&expr)?;
    Ok(())
  }

  fn handle_assign(&mut self, identifier: String, expr: &Meta<Expr>) -> Result<(), RuntimeError> {
    let val = self.run_expr(&expr)?;
    if let Some(table_for_scope) = self.symbol_table.last_mut() {
      table_for_scope.insert(
        identifier.clone(),
        SymbolEntry {
          name: identifier.clone(),
          value: SymbolVal::Object(val),
        },
      );
    }
    Ok(())
  }

  fn handle_function(
    &mut self,
    identifier: String,
    params: &Vec<String>,
    stmt: &Meta<Stmt>,
  ) -> Result<(), RuntimeError> {
    if let Some(table_for_scope) = self.symbol_table.last_mut() {
      table_for_scope.insert(
        identifier.clone(),
        SymbolEntry {
          name: identifier.clone(),
          value: SymbolVal::Function(params.clone(), stmt.clone()),
        },
      );
    }
    Ok(())
  }

  fn run_expr(&mut self, expr: &Meta<Expr>) -> Result<Object, RuntimeError> {
    match expr.inside {
      Expr::Binary(ref op, ref e1, ref e2) => self.handle_binary(op, e1, e2),
      Expr::Unary(ref op, ref e1) => self.handle_unary(op, e1),
      Expr::FunctionCall(ref name, ref args) => self.handle_function_call(name.to_string(), args),
      Expr::Identifier(ref name) => self.handle_identifier(expr, name),
      Expr::Number(num) => self.handle_number(Object::Number(num)),
      _ => self.error(
        format!("Couldn't handle expr: {:?}", expr),
        Some(expr.byte_offset),
      ),
    }
  }

  fn handle_binary(
    &mut self,
    operator: &Operator,
    expr1: &Meta<Expr>,
    expr2: &Meta<Expr>,
  ) -> Result<Object, RuntimeError> {
    let e1_num = get_number(self.run_expr(&expr1)?)?;
    let e2_num = get_number(self.run_expr(&expr2)?)?;
    let result = match operator {
      Operator::Multiply => e1_num * e2_num,
      Operator::Divide => e1_num / e2_num,
      Operator::Add => e1_num + e2_num,
      Operator::Subtract => e1_num - e2_num,
      Operator::Mod => e1_num % e2_num,
      _ => 0.0, // TODO: error
    };
    Ok(Object::Number(result))
  }

  fn handle_unary(
    &mut self,
    operator: &Operator,
    expr1: &Meta<Expr>,
  ) -> Result<Object, RuntimeError> {
    let e1_num = get_number(self.run_expr(&expr1)?)?;
    let result = match operator {
      Operator::Negate => -e1_num,
      _ => 0.0, // TODO: error
    };
    Ok(Object::Number(result))
  }

  fn handle_function_call(
    &mut self,
    identifier: String,
    exprs: &Vec<Meta<Expr>>,
  ) -> Result<Object, RuntimeError> {
    let result = if let Some(var) = get_var(&identifier, &self.symbol_table.clone()) {
      match var.value {
        SymbolVal::Function(ref params, ref stmt) => {
          // load params into symbol table
          for (i, expr) in exprs.iter().enumerate() {
            let param_name = params.get(i).unwrap().to_string(); // TODO: check error
            let expr_val = self.run_expr(expr)?;
            if let Some(table_for_scope) = self.symbol_table.last_mut() {
              table_for_scope.insert(
                param_name.clone(),
                SymbolEntry {
                  name: param_name.clone(),
                  value: SymbolVal::Object(expr_val),
                },
              );
            }
          }
          self.run_stmt(stmt)?; // TODO: get return val
          Ok(Object::Number(0.0))
        }
        SymbolVal::StdLib(ref name) => {
          let mut evaled_args = vec![];
          for expr in exprs {
            evaled_args.push(self.run_expr(expr)?);
          }
          self.run_stdlib_function(name, evaled_args)
        }
        SymbolVal::Object(ref object) => Ok(object.clone()),
      }
    } else {
      self.error(
        format!("Couldn't find function with name: {}", identifier),
        None,
      )
    };
    result
  }

  fn handle_identifier(&mut self, expr: &Meta<Expr>, name: &str) -> Result<Object, RuntimeError> {
    if let Some(var) = get_var(name, &self.symbol_table) {
      if let SymbolVal::Object(ref obj) = var.value {
        return Ok(obj.clone());
      }
    }
    self.error(
      format!("Couldn't find identifier: {}", name),
      Some(expr.byte_offset),
    )
  }

  fn handle_number(&mut self, num: Object) -> Result<Object, RuntimeError> {
    Ok(num)
  }

  fn run_stdlib_function(
    &self,
    function_name: &str,
    args: Vec<Object>,
  ) -> Result<Object, RuntimeError> {
    match function_name {
      "print" => stdlib::std_print(args),
      "Box" => stdlib::std_make_box(args),
      "Plane" => stdlib::std_make_plane(args),
      "move" => stdlib::std_move(args),
      "difference" => stdlib::std_difference(args),
      "rotate_x" => stdlib::std_rotate_x(args),
      "display" => stdlib::std_display(args),
      _ => self.error(
        format!("Couldn't find stdlib function with name: {}", function_name),
        None,
      ),
    }
  }

  fn add_stdlib(&mut self) {
    let toplevel = self.symbol_table.get_mut(0).unwrap();
    for std_lib_function in self.std_lib_functions.clone() {
      toplevel.insert(
        std_lib_function.to_string(),
        SymbolEntry {
          name: std_lib_function.to_string(),
          value: SymbolVal::StdLib(std_lib_function.to_string()),
        },
      );
    }
  }

  fn error(&self, msg: String, byte_offset: Option<usize>) -> Result<Object, RuntimeError> {
    let msg = if let Some(byte_offset) = byte_offset {
      let line = get_line_number(&self.source_code, byte_offset);
      format!("Runtime Error: {} at line {}", msg, line)
    } else {
      format!("Runtime Error: {}", msg)
    };
    Err(RuntimeError::new(msg))
  }
}

fn get_var<'a>(
  var_name: &str,
  symbol_table: &'a Vec<HashMap<String, SymbolEntry>>,
) -> Option<&'a SymbolEntry> {
  for table in symbol_table.iter().rev() {
    if let Some(symbol_entry) = table.get(var_name) {
      return Some(symbol_entry);
    }
  }
  None
}

pub fn get_number(object: Object) -> Result<f64, RuntimeError> {
  if let Object::Number(num) = object {
    Ok(num)
  } else {
    Err(RuntimeError::new(format!(
      "Object is not a number: {:?}",
      object
    )))
  }
}

pub fn get_solid(object: Object) -> Result<Solid, RuntimeError> {
  if let Object::Solid(solid) = object {
    Ok(solid)
  } else {
    Err(RuntimeError::new(format!(
      "Object is not a solid: {:?}",
      object
    )))
  }
}
