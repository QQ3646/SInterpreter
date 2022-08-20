use std::mem;
use crate::libs::{lex::TokenType, lex::TokenType::*, expr::ast::*};
use crate::libs::lex::{Token, Lox};

type BoxExpr = Option<Box<Expr>>;

pub struct ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

type ParseResult = Result<BoxExpr, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        self.expression()
    }

    fn expression(&mut self) -> ParseResult {
        self.equality()
    }

    // equality -> comparison (("!=" | "==" comparison)*
    fn equality(&mut self) -> Result<BoxExpr, ParseError> {
        let mut expr = match self.comparison() {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };

        while self.matching([BANG, BANG_EQUAL]) {
            let operator = self.previous().clone();
            expr = match self.comparison() {
                Ok(right) => Some(Box::new(Expr::Binary {
                    left: expr,
                    operator,
                    right,
                })),
                Err(e) => return Err(e),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<BoxExpr, ParseError> {
        let mut expr = match self.term() {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };

        while self.matching([GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous().clone();
            expr = match self.term() {
                Ok(right) => Some(Box::new(Expr::Binary {
                    left: expr,
                    operator,
                    right,
                })),
                Err(e) => return Err(e),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<BoxExpr, ParseError> {
        let mut expr = match self.factor() {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };

        while self.matching([MINUS, PLUS]) {
            let operator = self.previous().clone();
            expr = match self.factor() {
                Ok(right) => Some(Box::new(Expr::Binary {
                    left: expr,
                    operator,
                    right,
                })),
                Err(e) => return Err(e)
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<BoxExpr, ParseError> {
        let mut expr = match self.unary() {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };

        while self.matching([SLASH, STAR]) {
            let operator = self.previous().clone();
            expr = match self.unary() {
                Ok(right) => {
                    Some(Box::new(Expr::Binary {
                        left: expr,
                        operator,
                        right,
                    }))
                },
                Err(e) => return Err(e)
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<BoxExpr, ParseError> {
        if self.matching([BANG, MINUS]) {
            let operator = self.previous().clone();
            return match self.unary() {
                Ok(right) => {
                    Ok(Some(Box::new(Expr::Unary {
                        operator,
                        right,
                    })))
                },
                Err(e) => Err(e)
            }
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<BoxExpr, ParseError> {
        if self.matching([FALSE]) {
            Ok(Some(Box::new(Expr::Literal { value: Object::Bool(false) })))
        } else if self.matching([TRUE]) {
            Ok(Some(Box::new(Expr::Literal { value: Object::Bool(true) })))
        } else if self.matching([NIL]) {
            Ok(Some(Box::new(Expr::Literal { value: Object::Nil })))
        } else if self.matching([NUMBER, STRING]) {
            let prev = self.previous();
            match prev.token_type {
                STRING | NUMBER => Ok(Some(Box::new(Expr::Literal { value: prev.literal.to_object() }))),
                _ => panic!("Oh wow, that is more stranger thing")
            }
        } else if self.matching([LEFT_PAREN]) {
            let expression = match self.expression() {
                Ok(expression) => expression,
                Err(e) => return Err(e),
            };
            if let Err(e) = self.consume(RIGHT_PAREN, "Expect ')' after expression.") {
                return Err(e);
            }
            Ok(Some(Box::new(Expr::Grouping { expression })))
        } else {
            Err(self.error(self.peek(), "Expect expression."))
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<&Token, ParseError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), msg))
    }

    fn sync(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if let SEMICOLON = self.previous().token_type {
                return;
            }

            match self.peek().token_type {
                BOX | FUN | LET | FOR | IF | WHILE | PRINT | RETURN => return,
                _ => ()
            }

            self.advance();
        }
    }

    fn error(&self, token: &Token, msg: &str) -> ParseError {
        Self::report_error(token, msg);

        ParseError
    }

    fn report_error(token: &Token, msg: &str) {
        if let EOF = token.token_type {
            Lox::report_error(token.line, " at the end ", msg);
        } else {
            Lox::report_error(token.line, &format!(" at '{}'", token.lexeme), msg);
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        let current_token = self.peek();
        mem::discriminant(token_type) == mem::discriminant(&current_token.token_type)
    }

    fn is_at_end(&self) -> bool {
        if let EOF = self.peek().token_type {
            return true;
        }
        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    // Some unsecure things
    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).expect("Wow, that is strange shit")
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).expect("Wow, that is strange shit")
    }

    fn matching<const N: usize>(&mut self, tokens: [TokenType; N]) -> bool {
        for token in tokens {
            if self.check(&token) {
                self.advance();
                return true;
            }
        }
        false
    }
}