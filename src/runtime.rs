use compiler::ast::{Expr, Meta, Operator, Stmt};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

type Object = f64;

#[derive(Debug, Clone)]
pub struct CompilationError {
  msg: String,
}

impl CompilationError {
  fn new(msg: String) -> CompilationError {
    CompilationError { msg: msg }
  }
}

impl fmt::Display for CompilationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl Error for CompilationError {
  fn description(&self) -> &str {
    self.msg.as_str()
  }

  fn cause(&self) -> Option<&Error> {
    None
  }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
enum SymbolVal {
  Function(Vec<String>, Meta<Stmt>),
  StdLib(String),
  // Object(Box<Fn(dyn HashMap<String, Object>) -> Object>),
  Object(Object),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SymbolEntry {
  name: String,
  value: SymbolVal,
}

pub struct Runtime {
  symbol_table: Vec<HashMap<String, SymbolEntry>>,
}

impl Runtime {
  pub fn new() -> Runtime {
    Runtime {
      symbol_table: vec![HashMap::new()],
    }
  }

  pub fn compile(&mut self, program: &Vec<Meta<Stmt>>) -> Result<(), CompilationError> {
    for stmt in program {
      self.compile_stmt(stmt)?;
    }
    Ok(())
  }

  fn compile_stmt(&mut self, stmt: &Meta<Stmt>) -> Result<(), CompilationError> {
    match stmt.inside {
      Stmt::Block(ref stmts) => self.handle_block(stmts),
      Stmt::Return(ref expr) => self.handle_return(expr),
      Stmt::Expr(ref expr) => self.handle_expr(expr),
      Stmt::Assign(ref identifier, ref expr) => self.handle_assign(identifier.to_string(), expr),
      Stmt::Function(ref identifier, ref param, ref stmt) => {
        self.handle_function(identifier.to_string(), param, stmt)
      }
      _ => Ok(()),
    }
  }

  fn handle_block(&mut self, stmts: &Vec<Meta<Stmt>>) -> Result<(), CompilationError> {
    self.symbol_table.push(HashMap::new());
    for stmt in stmts {
      self.compile_stmt(&stmt)?;
    }
    self.symbol_table.pop();
    Ok(())
  }

  fn handle_return(&mut self, expr: &Meta<Expr>) -> Result<(), CompilationError> {
    println!("Returning: {:?}", self.compile_expr(&expr)?);
    Ok(())
  }

  fn handle_expr(&mut self, expr: &Meta<Expr>) -> Result<(), CompilationError> {
    self.compile_expr(&expr)?;
    Ok(())
  }

  fn handle_assign(
    &mut self,
    identifier: String,
    expr: &Meta<Expr>,
  ) -> Result<(), CompilationError> {
    let val = self.compile_expr(&expr)?;
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
  ) -> Result<(), CompilationError> {
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

  fn compile_expr(&mut self, expr: &Meta<Expr>) -> Result<Object, CompilationError> {
    match expr.inside {
      Expr::Binary(ref op, ref e1, ref e2) => self.handle_binary(op, e1, e2),
      Expr::Unary(ref op, ref e1) => self.handle_unary(op, e1),
      Expr::FunctionCall(ref name, ref args) => self.handle_function_call(name.to_string(), args),
      Expr::Identifier(ref name) => self.handle_identifier(name),
      Expr::Number(num) => self.handle_number(num),
      _ => Ok(0.0), // TODO: error
    }
  }

  fn handle_binary(
    &mut self,
    operator: &Operator,
    expr1: &Meta<Expr>,
    expr2: &Meta<Expr>,
  ) -> Result<Object, CompilationError> {
    let e1_val = self.compile_expr(&expr1)?;
    let e2_val = self.compile_expr(&expr2)?;
    let result = match operator {
      Operator::Multiply => e1_val * e2_val,
      Operator::Divide => e1_val / e2_val,
      Operator::Add => e1_val + e2_val,
      Operator::Subtract => e1_val - e2_val,
      Operator::Mod => e1_val % e2_val,
      _ => 0.0, // TODO: error
    };
    Ok(result)
  }

  fn handle_unary(
    &mut self,
    operator: &Operator,
    expr1: &Meta<Expr>,
  ) -> Result<Object, CompilationError> {
    let e1_val = self.compile_expr(&expr1)?;
    let result = match operator {
      Operator::Negate => -e1_val,
      _ => 0.0, // TODO: error
    };
    Ok(result)
  }

  fn handle_function_call(
    &mut self,
    identifier: String,
    exprs: &Vec<Meta<Expr>>,
  ) -> Result<Object, CompilationError> {
    let result = if let Some(var) = get_var(&identifier, &self.symbol_table.clone()) {
      match var.value {
        SymbolVal::Function(ref params, ref stmt) => {
          // load params into symbol table
          for (i, expr) in exprs.iter().enumerate() {
            let expr_val = self.compile_expr(expr)?;
            if let Some(table_for_scope) = self.symbol_table.last_mut() {
              table_for_scope.insert(
                identifier.clone(),
                SymbolEntry {
                  name: params.get(i).unwrap().to_string(), // TODO: check error
                  value: SymbolVal::Object(expr_val),
                },
              );
            }
          }
          self.compile_stmt(stmt)?;
          Ok(0.0)
        }
        SymbolVal::StdLib(ref name) => Ok(0.0),
        SymbolVal::Object(Object) => Ok(0.0), // TODO: error
      }
    } else {
      Ok(0.0)
    };
    result
  }

  fn handle_identifier(&mut self, name: &str) -> Result<Object, CompilationError> {
    if let Some(var) = get_var(name, &self.symbol_table) {
      if let SymbolVal::Object(obj) = var.value {
        return Ok(obj);
      }
    }
    // TODO: error
    Ok(0.0)
  }

  fn handle_number(&mut self, num: Object) -> Result<Object, CompilationError> {
    Ok(num)
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
