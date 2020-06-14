#![allow(non_snake_case)]

#[derive(Debug)]
pub enum Token {
    LeftParen, RightParen, Dot, Comma, Equals,

    Identifier(String),
    Number(usize),
    StringLiteral(String),

    EOF,
}

pub struct Lexer {
    tokens: Vec<Token>,
    source: String,
    start: usize,
    current: usize,
}

fn is_number(c: char) -> bool {
    return if c >= '0' && c <= '9' {
        true
    } else {
        false
    }
}

fn is_letter(c: char) -> bool {
    return if (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') {
        true
    } else {
        false
    }
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            tokens: Vec::new(),
            start: 0,
            current: 0,
            source: source,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.isAtEnd() {
            // we are at the beginning of the next lexeme
            self.start = self.current;
            self.scanToken();
        }

        self.tokens.push(Token::EOF);

        return &self.tokens
    }

    // scan one token
    fn scanToken(&mut self) {
        let c = self.advance();

        match c {
            c if c.is_whitespace() => (),
            '(' => self.addToken(Token::LeftParen),
            ')' => self.addToken(Token::RightParen),
            '.' => self.addToken(Token::Dot),
            ',' => self.addToken(Token::Comma),
            '=' => self.addToken(Token::Equals),
            '\'' => self.string_literal(),
            c if is_letter(c) => self.identifier(),
            c if is_number(c) => self.number(),
            _ => {
                panic!("Unexpected character: {}", c)
            }
        }
    }

    fn string_literal(&mut self) {
        while is_letter(self.peek()) {
            self.advance();
        }

        self.addToken(Token::StringLiteral(
            self.source[self.start + 1 ..self.current].to_string())
        );

        self.advance();
    }

    fn identifier(&mut self) {
        while is_letter(self.peek()) {
            self.advance();
        }

        self.addToken(Token::Identifier(
            self.source[self.start ..self.current].to_string())
        );
    }

    fn number(&mut self) {
        while is_number(self.peek()) {
            self.advance();
        }

        self.addToken(Token::Number(
            self.source[self.start .. self.current].parse().unwrap())
        );
    }

    // returns character without consuming it
    fn peek(&self) -> char {
        return if self.isAtEnd() {
            '\0'
        } else {
            self.nth(self.current)
        }
    }

    fn isAtEnd(&self) -> bool {
        self.current >= self.source.len()
    }

    // advance the pointer and return the new current char
    fn advance(&mut self) -> char {
        self.current += 1;
        self.nth(self.current - 1)
    }

    // returns nth char in source
    fn nth(&self, n: usize) -> char {
        self.source.as_bytes()[n] as char
    }

    fn addToken(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

/*
Int.Fuction('fib', args=Int.New('i'), opts=(Int.New('a'), Int.New('b')), body=(
    (a.isUnset().then(a.set(0))),
    (b.isUnset().then(b.set(1))),
    (i.isEqual(0).then(this.exit(a), otherwise=this.exit(this(i.sub(1), b, a.add(b)))))
))
*/