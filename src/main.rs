mod ast;
mod parse;

extern crate pest;
#[macro_use]
extern crate pest_derive;


fn main() {
    let input = "a.b.c
    ";
    let res = parse::parse(input);
    
    match res {
        Ok(ast) => {
            println!("{:?}", ast);
        }
        Err(err) => {
            println!("Syntax error:");
            println!("{}", err);
        }
    }

}
