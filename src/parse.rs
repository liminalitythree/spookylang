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
        ast.push(exp_r(expr));
    }

    Ok(ast)
}

fn exp_r(pair: Pair<Rule>) -> Expr {
    let pairs = pair.into_inner();
    for p in pairs {
        println!("{:?}", p);
    }
    Expr::Integer(2)
    /*match pair.as_rule() {
        Rule::number | Rule::string | Rule::decimal => {
            literal(pair)
        }

        Rule::call => {
            call(pair)
        }

        unknown_term => panic!("Unexpected term: {:?}", unknown_term),
    }*/
}
/*
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
            Expr::StringLiteral(pair.as_str().to_string())
        }

        unknown_term => panic!("Unexpected term: {:?}", unknown_term),
    }
}

fn call(pair: Pair<Rule>) -> Expr {
    let pairs = pair.into_inner();

    let mut ident: Vec<String> = Vec::new();
    let mut args: Option<Vec<Arg>> = None;

    for p in pairs {
        match p.as_rule() {
            Rule::identifier => {
                ident.push(p.as_str().to_string());
            }
            
            Rule::callArgs => {
                args = Some(call_args(p));
            }

            unknown_term => panic!("Unexpected term: {:?}", unknown_term),
        }
    }
    Expr::Call(ident, args.expect("call parsing had an oopsie maybe"))
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
    let mut ident: Option<String> = None;
    let mut expr: Option<Expr> = None;

    let pairs = pair.into_inner();

    for p in pairs {
        match p.as_rule() {
            Rule::identifier => {
                ident = Some(p.as_str().to_string())
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
*/