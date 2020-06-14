mod lexer;
mod parse_ast;
mod parser;

fn main() {
    let input = "Int.Fuction('fib', args=Int.New('i'), opts=(Int.New('a'), Int.New('b')), body=(
        (a.isUnset().then(a.set(0))),
        (b.isUnset().then(b.set(1))),
        (i.isEqual(0).then(this.exit(a), otherwise=this.exit(this(i.sub(1), b, a.add(b)))))
    ))
    ";
    let mut lexer = lexer::Lexer::new(input.to_string());
    let tokens = lexer.scan_tokens();

    println!("{:?}", tokens);
}
