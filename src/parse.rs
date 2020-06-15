use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;
use crate::ast::{Arg, Expr};

#[derive(Parser)]
#[grammar = "spooky.pest"]
struct SpookyParser;

pub fn parse(source: &str) -> Result<Vec<Expr>, Error<Rule>> {
    let mut ast: Vec<Expr> = Vec::new();

    let exprs = SpookyParser::parse(Rule::program, source)?;
    for expr in exprs {
        if expr.as_rule() != Rule::EOI {
            ast.push(exp_r(expr));
        }
    }

    Ok(ast)
}

fn exp_r(pair: Pair<Rule>) -> Expr {
    let pairs = pair.into_inner();
    let mut exprs: Vec<Expr> = Vec::new();

    for p in pairs {
        let rule = p.as_rule();
        match rule {
            Rule::number | Rule::string | Rule::decimal => {
                exprs.push(literal(p));
            }

            Rule::identifier => {
                exprs.push(Expr::Identifier(p.as_str()));
            }

            Rule::callArgs => {
                let args = call_args(p);
                let call = Expr::Call(exprs, args);
                exprs = Vec::new();
                exprs.push(call);
            }

            unknown_term => panic!("Unexpected term: {:?}", unknown_term),
        }
    }

    return match exprs.len() {
        1 => {
            exprs.remove(0)
        }

        q => {
            panic!("the parser seems like it had a bug? internal expers representation was {} at the end.", q);
        }
    }
}

fn literal(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::number => {
            let istr = pair.as_str();
            let integer: i32 = istr.parse().unwrap();
            Expr::Integer(integer)
        }

        Rule::decimal => {
            let dstr = pair.as_str();
            let flt: f32 = dstr.parse().unwrap();
            Expr::Float(flt)
        }

        Rule::string => {
            Expr::StringLiteral(pair.as_str())
        }

        unknown_term => panic!("Unexpected term: {:?}", unknown_term),
    }
}

fn call_args(pair: Pair<Rule>) -> Vec<Arg> {
    let pairs = pair.into_inner();
    let mut args: Vec<Arg> = Vec::new();

    for p in pairs {
        let arg = match p.as_rule() {
            Rule::namedArg => named_arg(p),
            _ => Arg::Expr(exp_r(p)),
        };
        args.push(arg);
    }
    
    args
}

fn named_arg(pair: Pair<Rule>) -> Arg {
    let mut ident: Option<&str> = None;
    let mut expr: Option<Expr> = None;

    let pairs = pair.into_inner();

    for p in pairs {
        match p.as_rule() {
            Rule::identifier => {
                ident = Some(p.as_str())
            }

            _ => {
                expr = Some(exp_r(p))
            }
        }
    }

    Arg::NamedArg(
        ident.expect("namedArg parsing had an oopsie. maybe"),
        expr.expect("namedArg parsing had an oopsie.")
    )
}
