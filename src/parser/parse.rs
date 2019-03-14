use super::ast::{Meta, Stmt};
use super::util::get_line_number;
use lalrpop_util::ParseError;

lalrpop_mod!(pub grammar, "/parser/grammar.rs");

/// Parses a program and alerts on parse errors
pub fn parse_program(program_string: &String) -> Option<Vec<Meta<Stmt>>> {
  let maybe_ast = grammar::ProgramParser::new().parse(program_string);
  match maybe_ast {
    Ok(ast) => Some(ast),
    Err(e) => {
      match &e {
        ParseError::UnrecognizedToken { token, .. } => {
          if let Some((byte, ..)) = token {
            let line = get_line_number(program_string, byte.clone());
            println!("Syntax Error at line: {}", line);
          } else {
            println!("Syntax error: {:?}", e);
          }
        }
        ParseError::InvalidToken { location } => {
          let line = get_line_number(program_string, location.clone());
          println!("Invalid token at line {}", line);
        }
        misc @ _ => println!("{:?}", misc),
      }
      None
    }
  }
}
