use crate::lexer::Token;

/*
program = expr+
expr    = call | literal
call    = identifier ( "." identifier )* "(" args? ")"
args    = argexpr ("," argexpr)*
argexpr = expr | identifier "=" expr
literal = number | stringliteral
*/

pub type Ast = Vec<Expr>;

pub enum Expr {
    Call(FunctionCall),
    Literal(Literal),
}

pub enum Literal {
    String(String),
    Number(f32),
}

pub struct Identifier {
    idents: Vec<String>,
}

pub struct FunctionCall {
    identifier: Identifier,
    args: Option<Vec<Arg>>,
}

pub enum Arg {
    Arg(Expr),
    NamedArg(NamedArg),
}

pub struct NamedArg {
    identifier: Identifier,
    expr: Expr,
}

pub struct Node<T> {
    value: T,
    token: Token,
}
