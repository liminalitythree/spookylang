#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    Float(f32),
    StringLiteral(String),
    
    Call(Vec<Expr>, Vec<Arg>)
}

#[derive(Debug)]
pub enum Arg {
    Expr(Expr),
    NamedArg(String, Expr),
}
