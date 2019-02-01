use super::ast::{Meta, Stmt};
use super::util::get_line_number;
use lalrpop_util::ParseError;
use std::io;
use std::io::BufRead;

lalrpop_mod!(pub grammar, "/compiler/grammar.rs");

/// Compiles and prints an oc program from stdin
pub fn compile() -> Option<Vec<Meta<Stmt>>> {
  let stdin = io::stdin();
  let program_string = stdin
    .lock()
    .lines()
    .filter_map(|l| l.ok())
    .collect::<Vec<_>>()
    .join("\n");
  parse_program(&program_string)
}

/// Parses a program and alerts on parse errors
fn parse_program(program_string: &String) -> Option<Vec<Meta<Stmt>>> {
  let maybe_ast = grammar::ProgramParser::new().parse(program_string);
  match maybe_ast {
    Ok(ast) => Some(ast),
    Err(e) => {
      match e {
        ParseError::UnrecognizedToken { token, .. } => {
          if let Some((byte, ..)) = token {
            let line = get_line_number(program_string, byte);
            println!("Syntax Error at line: {}", line);
          }
        }
        ParseError::InvalidToken { location } => {
          let line = get_line_number(program_string, location);
          println!("Invalid token at line {}", line);
        }
        misc @ _ => println!("{:?}", misc),
      }
      None
    }
  }
}
