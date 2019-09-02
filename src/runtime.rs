use parser::ast::{Expr, Meta, Operator, Stmt};
use parser::util::get_line_number;
use solid::{Edge, Face, Plane, Point, Solid, Vector};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;
use stdlib;

const CURRENT_FUNCTION_CALL_KEY: &'static str = "___CURRENT_FUNCTION_CALL";

lazy_static! {
  // Note: these need to be hooked up to the actual definitions in
  // stdlib.rs in Runtime.run_stdlib_function_call()
  static ref STD_LIB_FUNCTIONS: Vec<&'static str> = vec![
    "print",
    "Box",
    "Plane",
    "move",
    "difference",
    "rotate_x",
    "display",
    "write_stl",
  ];
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
  Number(f64),
  Str(String),
  Point(Point),
  Edge(Edge),
  Plane(Plane),
  Face(Face),
  Vector(Vector),
  Solid(Solid),
  List(Vec<Object>),
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

  fn cause(&self) -> Option<&dyn Error> {
    None
  }
}

#[derive(Debug, Clone, PartialEq)]
enum SymbolVal {
  Function(Vec<String>, Meta<Stmt>),
  StdLib(String),
  Object(Object),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarEntry {
  name: String,
  value: SymbolVal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionMetadataEntry {
  return_val: Option<Object>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolEntry {
  Variable(VarEntry),
  Function(FunctionMetadataEntry),
}

pub struct Runtime {
  symbol_table: Vec<HashMap<String, SymbolEntry>>,
  source_code: String,
  stdout: Box<dyn Write>,
}

impl Runtime {
  pub fn new(source_code: String, stdout: Option<Box<dyn Write>>) -> Runtime {
    Runtime {
      symbol_table: vec![HashMap::new()],
      source_code: source_code,
      stdout: stdout.unwrap_or(Box::new(io::stdout()) as Box<dyn Write>),
    }
  }

  // Runs a program
  pub fn run(&mut self, program: &Vec<Meta<Stmt>>) -> Result<(), RuntimeError> {
    self.add_stdlib();
    for stmt in program {
      self.run_stmt(stmt)?;
    }
    Ok(())
  }

  // Runs any AST statement
  fn run_stmt(&mut self, stmt: &Meta<Stmt>) -> Result<(), RuntimeError> {
    match stmt.inside {
      Stmt::Block(ref stmts) => self.handle_block(stmts),
      Stmt::Return(ref expr) => self.handle_return(expr),
      Stmt::Expr(ref expr) => self.handle_expr(expr),
      Stmt::Assign(ref identifier, ref expr) => self.handle_assign(identifier.to_string(), expr),
      Stmt::Function(ref identifier, ref params, ref stmt) => {
        self.handle_function_declaration(identifier.to_string(), params, stmt)
      }
      Stmt::For(ref assign, ref condition, ref inc, ref body) => {
        self.handle_for(assign, condition, inc, body)
      }
      Stmt::If(ref condition, ref body) => self.handle_if(condition, body),
    }
  }

  // Runs an AST for loop
  fn handle_for(
    &mut self,
    assign: &Meta<Stmt>,
    cond: &Meta<Expr>,
    inc: &Meta<Stmt>,
    body: &Meta<Stmt>,
  ) -> Result<(), RuntimeError> {
    self.run_stmt(assign)?;
    while get_number(self.run_expr(cond)?)? > 0.0 {
      self.run_stmt(body)?;
      self.run_stmt(inc)?;
    }
    Ok(())
  }

  // Runs an AST if statement
  fn handle_if(&mut self, cond: &Meta<Expr>, body: &Meta<Stmt>) -> Result<(), RuntimeError> {
    if get_number(self.run_expr(cond)?)? > 0.0 {
      self.run_stmt(body)?;
    }
    Ok(())
  }

  // Processes an AST block, running any statements within
  fn handle_block(&mut self, stmts: &Vec<Meta<Stmt>>) -> Result<(), RuntimeError> {
    self.symbol_table.push(HashMap::new());
    for stmt in stmts {
      self.run_stmt(&stmt)?;
    }
    self.symbol_table.pop();
    Ok(())
  }

  // Runs an AST return statement, inserting the value into the symbol table
  // as the function return
  fn handle_return(&mut self, expr: &Meta<Expr>) -> Result<(), RuntimeError> {
    let return_val = self.run_expr(expr)?;

    // insert return val in closest function call
    for table in self.symbol_table.iter_mut().rev() {
      if let Some(SymbolEntry::Function(_)) = table.get(CURRENT_FUNCTION_CALL_KEY) {
        table.insert(
          CURRENT_FUNCTION_CALL_KEY.to_string(),
          SymbolEntry::Function(FunctionMetadataEntry {
            return_val: Some(return_val),
          }),
        );
        break;
      }
    }
    Ok(())
  }

  // Runs an AST expr statement
  fn handle_expr(&mut self, expr: &Meta<Expr>) -> Result<(), RuntimeError> {
    self.run_expr(&expr)?;
    Ok(())
  }

  // Processes an AST assignment statement
  fn handle_assign(&mut self, identifier: String, expr: &Meta<Expr>) -> Result<(), RuntimeError> {
    let val = self.run_expr(&expr)?;
    if let Some(table_for_scope) = self.symbol_table.last_mut() {
      table_for_scope.insert(
        identifier.clone(),
        SymbolEntry::Variable(VarEntry {
          name: identifier.clone(),
          value: SymbolVal::Object(val),
        }),
      );
    }
    Ok(())
  }

  // Processes an AST function declaration
  fn handle_function_declaration(
    &mut self,
    identifier: String,
    params: &Vec<String>,
    stmt: &Meta<Stmt>,
  ) -> Result<(), RuntimeError> {
    if let Some(table_for_scope) = self.symbol_table.last_mut() {
      table_for_scope.insert(
        identifier.clone(),
        SymbolEntry::Variable(VarEntry {
          name: identifier.clone(),
          value: SymbolVal::Function(params.clone(), stmt.clone()),
        }),
      );
    }
    Ok(())
  }

  // Runs an AST expr
  fn run_expr(&mut self, expr: &Meta<Expr>) -> Result<Object, RuntimeError> {
    match expr.inside {
      Expr::Binary(ref op, ref e1, ref e2) => self.handle_binary(op, e1, e2),
      Expr::Unary(ref op, ref e1) => self.handle_unary(op, e1),
      Expr::FunctionCall(ref name, ref args) => {
        self.handle_function_call(expr, name.to_string(), args)
      }
      Expr::Identifier(ref name) => self.handle_identifier(expr, name),
      Expr::Number(num) => self.handle_number(Object::Number(num)),
      Expr::Str(ref s) => self.handle_str(Object::Str(s.clone())),
      Expr::List(ref l) => self.handle_list(l),
    }
  }

  // Processes an AST binary operator
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

  // Processes an AST unary operator
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

  // Processes and runs a function call (may be a stdlib function call
  // or a function defined in source code)
  fn handle_function_call(
    &mut self,
    expr: &Meta<Expr>,
    identifier: String,
    exprs: &Vec<Meta<Expr>>,
  ) -> Result<Object, RuntimeError> {
    if let Some(var) = get_var(&identifier, &self.symbol_table.clone()) {
      match var.value {
        SymbolVal::Function(ref params, ref stmt) => {
          self.handle_language_function_call(expr, exprs, params, stmt)
        }
        SymbolVal::StdLib(ref name) => {
          let mut evaled_args = vec![];
          for expr in exprs {
            evaled_args.push(self.run_expr(expr)?);
          }
          self.run_stdlib_function_call(name, evaled_args)
        }
        SymbolVal::Object(..) => self.error(
          format!("Object is not a function: {:?}", identifier),
          Some(expr.byte_offset),
        ),
      }
    } else {
      self.error(
        format!("Couldn't find function with name: {}", identifier),
        Some(expr.byte_offset),
      )
    }
  }

  // Runs a function defined in code (as opposed to stdlib) and returns
  // the result as an Object
  fn handle_language_function_call(
    &mut self,
    call_expr: &Meta<Expr>,
    exprs: &Vec<Meta<Expr>>,
    params: &Vec<String>,
    stmt: &Meta<Stmt>,
  ) -> Result<Object, RuntimeError> {
    if exprs.len() != params.len() {
      return self.error(
        format!("Number of expr args doesn't match number of params"),
        Some(call_expr.byte_offset),
      );
    }

    // add new scope level for function call
    let mut symbol_entry = HashMap::new();
    symbol_entry.insert(
      CURRENT_FUNCTION_CALL_KEY.to_string(),
      SymbolEntry::Function(FunctionMetadataEntry {
        return_val: Some(Object::Number(3.14)),
      }),
    );
    self.symbol_table.push(symbol_entry);

    // load params into symbol table
    for (i, expr) in exprs.iter().enumerate() {
      let param_name = params.get(i).unwrap().to_string(); // TODO: check error
      let expr_val = self.run_expr(expr)?;
      if let Some(table_for_scope) = self.symbol_table.last_mut() {
        table_for_scope.insert(
          param_name.clone(),
          SymbolEntry::Variable(VarEntry {
            name: param_name.clone(),
            value: SymbolVal::Object(expr_val),
          }),
        );
      }
    }

    self.run_stmt(stmt)?;

    let return_val = get_function(CURRENT_FUNCTION_CALL_KEY, &self.symbol_table)
      .unwrap()
      .clone()
      .return_val
      .unwrap_or(Object::Number(0.0));
    self.symbol_table.pop();
    Ok(return_val)
  }

  // Processes an AST identifier
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

  // Processes an AST number
  fn handle_number(&mut self, num: Object) -> Result<Object, RuntimeError> {
    Ok(num)
  }

  // Processes an AST str
  fn handle_str(&mut self, s: Object) -> Result<Object, RuntimeError> {
    Ok(s)
  }

  // Processes an AST list
  fn handle_list(&mut self, l: &Vec<Meta<Expr>>) -> Result<Object, RuntimeError> {
    let mut evaled_exprs = vec![];
    for ref expr in l {
      evaled_exprs.push(self.run_expr(expr)?);
    }

    Ok(Object::List(evaled_exprs))
  }

  // Runs a stdlib functions and returns the result as an Object
  fn run_stdlib_function_call(
    &mut self,
    function_name: &str,
    args: Vec<Object>,
  ) -> Result<Object, RuntimeError> {
    match function_name {
      "print" => stdlib::std_print(&mut self.stdout, args),
      "Box" => stdlib::std_make_box(args),
      "Plane" => stdlib::std_make_plane(args),
      "move" => stdlib::std_move(args),
      "difference" => stdlib::std_difference(args),
      "rotate_x" => stdlib::std_rotate_x(args),
      "display" => stdlib::std_display(args),
      "write_stl" => stdlib::std_write_stl(args),
      _ => self.error(
        format!("Couldn't find stdlib function with name: {}", function_name),
        None,
      ),
    }
  }

  // Inserts stdlib functions into the top level of the
  // symbol table
  fn add_stdlib(&mut self) {
    let toplevel = self.symbol_table.get_mut(0).unwrap();
    for std_lib_function in STD_LIB_FUNCTIONS.iter() {
      toplevel.insert(
        std_lib_function.to_string(),
        SymbolEntry::Variable(VarEntry {
          name: std_lib_function.to_string(),
          value: SymbolVal::StdLib(std_lib_function.to_string()),
        }),
      );
    }
  }

  // Generates a runtime error specifying the line number in the source code
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

// Searches a symbol table for a variable matching the
// given name
fn get_var<'a>(
  var_name: &str,
  symbol_table: &'a Vec<HashMap<String, SymbolEntry>>,
) -> Option<&'a VarEntry> {
  for table in symbol_table.iter().rev() {
    if let Some(SymbolEntry::Variable(var)) = table.get(var_name) {
      return Some(var);
    }
  }
  None
}

// Searches a symbol table for a function matching the
// given name
fn get_function<'a>(
  func_name: &str,
  symbol_table: &'a Vec<HashMap<String, SymbolEntry>>,
) -> Option<&'a FunctionMetadataEntry> {
  for table in symbol_table.iter().rev() {
    if let Some(SymbolEntry::Function(func)) = table.get(func_name) {
      return Some(func);
    }
  }
  None
}

// Extracts a number from an Object
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

// Extracts a solid from an Object
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

// Extracts a str from an Object
pub fn get_str(object: Object) -> Result<String, RuntimeError> {
  if let Object::Str(solid) = object {
    Ok(solid)
  } else {
    Err(RuntimeError::new(format!(
      "Object is not a string: {:?}",
      object
    )))
  }
}
