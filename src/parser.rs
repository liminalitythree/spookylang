use crate::parse_ast::*;
use crate::lexer::Token;

pub struct Parser<'a> {
    // points to current token in self.tokens
    current: usize,
    tokens: &'a Vec<Token>
}

/*
program = expr+
expr    = call | literal
call    = identifier ( "." identifier )* "(" args? ")"
args    = argexpr ("," argexpr)*
argexpr = expr | identifier "=" expr
literal = number | stringliteral
*/

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser {
        Parser {
            current: 0,
            tokens,
        }
    }

    fn program(&self) -> Vec<Expr> {
        let mut v: Vec<Expr> = Vec::new();

        while !self.isAtEnd() {
            v.push(self.expr());
        }

        return v;
    }

    fn expr(&self) -> Expr {
        return match self.peek() {
            Token::Number(a) => Expr::Literal(Literal::Number(*a as f32)),
            Token::StringLiteral(a) => Expr::Literal(Literal::String(a.to_string())),
            _ => self.call(),
        }
    }

    fn call(&self) -> Expr {
        let idents: Vec<String> = Vec::new();

        let a = self.consume(Token::Identifier("".to_string()), "Expected an identifier.");
        if let Token::Identifier(s) = a {
            idents.push(s.to_string());
        }

        while self.check(Token::Dot) {
            let a = self.consume(Token::Identifier("".to_string()), "Expected an identifier.");
            if let Token::Identifier(s) = a {
                idents.push(s.to_string());
            }    
        }

        // args
        self.consume(Token::LeftParen, "Expected opening parenthesis after function name.");

        let args: Option<Vec<Arg>> = if self.check(Token::RightParen) {
            None
        } else {
            Some(self.args())
        };

        self.consume(Token::RightParen, "Expected closing parenthesis after argument list.");

        Expr::Call(FunctionCall{
            identifier: 
        })
    }

    // if current token is type, consume it and return true
    // else, false
    fn mat_ch(&mut self, token_type: Token) -> bool {
        return if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    // if current token is type, current token
    // else, cause error with string message
    fn consume(&mut self, token_type: Token, message: &str) -> &Token {
        let t = self.peek();
        return if self.check(token_type) {
            self.advance()
        } else {
            panic!("{}", message);
        }
    }

    // if current token is token_type
    fn check(&self, token_type: Token) -> bool {
        use std::mem::discriminant;
        discriminant(self.peek()) == discriminant(&token_type)
    }

    // gets the current token and consumes it
    fn advance(&mut self) -> &Token {
        if !self.isAtEnd() {
            self.current += 1;
        }

        self.previous()
    }

    // if parser is at end of input
    fn isAtEnd(&self) -> bool {
        return match self.peek() {
            Token::EOF => true,
            _ => false,
        }
    }

    // gets current token without consuming it
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    // the previous token
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
