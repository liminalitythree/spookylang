#[derive(Debug)]
pub enum Expr<'a> {
    Integer(i32),
    Float(f32),
    StringLiteral(&'a str),

    Identifier(&'a str),
    Call(Vec<Expr<'a>>, Vec<Arg<'a>>),
}

#[derive(Debug)]
pub enum Arg<'a> {
    Expr(Expr<'a>),
    NamedArg(&'a str, Expr<'a>),
}
