pub type Program = Vec<Meta<Stmt>>;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Meta<T> {
  pub inside: T,
  pub byte_offset: usize,
}

impl<T> Meta<T> {
  pub fn new(inside: T, byte_offset: usize) -> Meta<T> {
    Meta {
      inside: inside,
      byte_offset: byte_offset,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Expr {
  Binary(Operator, Box<Meta<Expr>>, Box<Meta<Expr>>),
  Unary(Operator, Box<Meta<Expr>>),
  Number(f64),
  Str(String),
  Identifier(String),
  FunctionCall(String, Vec<Meta<Expr>>),
  List(Vec<Meta<Expr>>),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Stmt {
  Block(Vec<Meta<Stmt>>),
  If(Meta<Expr>, Box<Meta<Stmt>>),
  For(
    Box<Meta<Stmt>>,
    Meta<Expr>,
    Box<Meta<Stmt>>,
    Box<Meta<Stmt>>,
  ),
  Return(Meta<Expr>),
  Expr(Meta<Expr>),
  Function(String, Vec<String>, Box<Meta<Stmt>>),
  Assign(String, Meta<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum Operator {
  Multiply,
  Divide,
  Add,
  Subtract,
  Mod,
  Negate,
}
