use crate::libs::expr::ast::Object;
use std::collections::HashMap;
use std::fmt::Debug;
use std::{env, fs, io};
use crate::libs::expr::visitor::Visitor;
use crate::libs::parser::Parser;

#[derive(Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Nil,
}

impl ToString for LiteralValue {
    fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(n) => n.to_string(),
            LiteralValue::String(s) => s.clone(),
            LiteralValue::Nil => "Nil".to_string(),
        }
    }
}

impl LiteralValue {
    pub fn to_object(&self) -> Object {
        match self {
            LiteralValue::Number(n) => Object::Number(*n),
            LiteralValue::String(s) => Object::Str(s.clone()),
            LiteralValue::Nil => Object::Nil,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum TokenType {
    // Single characters tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One/two characters tokens
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals
    IDENTIFIER,
    STRING,
    NUMBER,

    // Some keywords
    BOX,
    ELSE,
    FUN,
    FOR,
    IF,
    OR,
    PRINT,
    RETURN,
    SUPER,
    SELF,
    TRUE,
    FALSE,
    AND,
    LET,
    WHILE,
    NIL,

    EOF,
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: LiteralValue,
    pub(crate) line: usize,
    pub lexeme: String,
}

impl Token {
    fn new(token_type: TokenType, literal: LiteralValue, line: usize, lexeme: String) -> Self {
        Self {
            token_type,
            literal,
            line,
            lexeme,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.token_type.to_string(),
            self.lexeme,
            self.literal.to_string()
        )
    }
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::LEFT_PAREN => "(",
            TokenType::RIGHT_PAREN => ")",
            TokenType::LEFT_BRACE => "{",
            TokenType::RIGHT_BRACE => "}",
            TokenType::COMMA => ",",
            TokenType::DOT => ".",
            TokenType::MINUS => "-",
            TokenType::PLUS => "+",
            TokenType::SEMICOLON => ";",
            TokenType::SLASH => "/",
            TokenType::STAR => "*",
            TokenType::BANG => "!",
            TokenType::BANG_EQUAL => "!=",
            TokenType::EQUAL => "=",
            TokenType::EQUAL_EQUAL => "==",
            TokenType::GREATER => ">",
            TokenType::GREATER_EQUAL => ">=",
            TokenType::LESS => "<",
            TokenType::LESS_EQUAL => "<=",
            TokenType::IDENTIFIER => "identifier",
            TokenType::STRING => "String",
            TokenType::NUMBER => "Number",
            TokenType::BOX => "box",
            TokenType::ELSE => "else",
            TokenType::FUN => "fun",
            TokenType::FOR => "for",
            TokenType::IF => "if",
            TokenType::OR => "or",
            TokenType::PRINT => "print",
            TokenType::RETURN => "return",
            TokenType::SUPER => "super",
            TokenType::SELF => "self",
            TokenType::TRUE => "true",
            TokenType::FALSE => "false",
            TokenType::AND => "and",
            TokenType::LET => "let",
            TokenType::WHILE => "while",
            TokenType::NIL => "nil",
            TokenType::EOF => "EOF",
        }
            .to_string()
    }
}

pub struct Lox {
    file: String,

    start_pos: usize,
    current_pos: usize,
    gotten_error: bool,
    line: usize,

    keywords: HashMap<&'static str, TokenType>,
}

impl Lox {
    pub fn init() -> Result<Self, io::Error> {
        let file_name = env::args().nth(1).unwrap_or_else(|| {
            println!("No input file name. \"main.slsf\" will be used instead.");
            //slsf - simple language source file
            "main.slsf".to_string()
        });
        let file = fs::read_to_string(file_name)?;
        let keywords: HashMap<&'static str, TokenType> = HashMap::from([
            ("or", TokenType::OR),
            ("and", TokenType::AND),
            ("box", TokenType::BOX),
            ("else", TokenType::ELSE),
            ("if", TokenType::IF),
            ("fun", TokenType::FUN),
            ("print", TokenType::PRINT),
            ("return", TokenType::RETURN),
            ("super", TokenType::SUPER),
            ("self", TokenType::SELF),
            ("true", TokenType::TRUE),
            ("false", TokenType::FALSE),
            ("let", TokenType::LET),
            ("while", TokenType::WHILE),
            ("nil", TokenType::NIL),
        ]);

        Ok(Self {
            file,

            start_pos: 0,
            current_pos: 0,
            gotten_error: false,
            line: 1,

            keywords,
        })
    }

    fn error(&mut self, line: usize, message: &str) {
        self.gotten_error = true;
        Self::report_error(line, "", message);
    }

    pub fn report_error(line: usize, where_: &str, message: &str) {
        eprintln!("[line {line}] Error {where_}: {message}");
    }

    pub fn run(&mut self) {
        let tokens = self.get_token_list();

        println!("Tokens count: {}", tokens.len());
        for token in &tokens {
            println!("{}", token.to_string())
        }
        if self.gotten_error {
            return;
        }
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        if let Ok(expr) = expr {
            let mut ast_printer = crate::libs::ast_printer::AstPrinter {};

            println!("{}", ast_printer.visit_expr(&expr).unwrap());
        }
    }

    fn scan_identifier(&mut self) -> TokenType {
        let start = self.current_pos;
        while let Some('0'..='9' | 'a'..='z' | 'A'..='Z' | '_') = self.peek_by(0) {
            self.advance();
        }

        let text = self.file[start - 1..self.current_pos].to_string();
        let token_option = self.keywords.get(&text as &str);
        (if let None = token_option {
            return TokenType::IDENTIFIER;
        });
        *token_option.unwrap()
    }

    fn scan_string(&mut self) -> TokenType {
        while let Some(s) = self.peek_by(0) {
            if s == '"' {
                break;
            } else if s == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.current_pos >= self.file.len() {
            self.error(self.line, "Unterminated string.");
        }
        self.advance();

        TokenType::STRING
    }

    fn scan_number(&mut self) -> TokenType {
        while let Some('0'..='9') = self.peek_by(0) {
            self.advance();
        }

        if let Some('.') = self.peek_by(0) {
            if let Some('0'..='9') = self.peek_by(1) {
                self.advance();

                while let Some('0'..='9') = self.advance() {}
            }
        }

        TokenType::NUMBER
    }

    fn matching(&mut self, expect: char) -> bool {
        if let Some(e) = self.file.chars().nth(self.current_pos) {
            if e == expect {
                self.current_pos += 1;
                return true;
            }
        }
        false
    }

    fn peek_by(&self, pos: usize) -> Option<char> {
        self.file.chars().nth(self.current_pos + pos)
    }

    fn advance(&mut self) -> Option<char> {
        self.current_pos += 1;
        self.file.chars().nth(self.current_pos - 1)
    }

    fn advance_by(&mut self, n: usize) -> Option<char> {
        self.current_pos += n;
        self.file.chars().nth(self.current_pos - n)
    }

    // fn add_token(list: &mut Vec<TokenType>, token_type: TokenType) {
    //     list.push(token_type);
    // }

    fn add_token(&mut self, list: &mut Vec<Token>, token_type: TokenType) {
        let lexeme = self.file[self.start_pos..self.current_pos].to_string();
        let literal = match token_type {
            TokenType::NUMBER => LiteralValue::Number(
                self.file[self.start_pos..self.current_pos]
                    .parse::<f64>()
                    .unwrap_or_else(|_| {
                        self.error(self.line, "Failed to parse number.");
                        f64::NAN
                    }),
            ),
            TokenType::STRING => LiteralValue::String(
                self.file[self.start_pos + 1..self.current_pos - 1].to_string(),
            ),
            _ => LiteralValue::Nil,
        };
        list.push(Token {
            token_type,
            literal,
            lexeme,
            line: self.line,
        })
    }

    fn get_token_list(&mut self) -> Vec<Token> {
        let mut list: Vec<Token> = Vec::new();
        while let Some(a) = self.advance() {
            self.start_pos = self.current_pos - 1;
            match a {
                '(' => self.add_token(&mut list, TokenType::LEFT_PAREN),
                ')' => self.add_token(&mut list, TokenType::RIGHT_PAREN),
                '{' => self.add_token(&mut list, TokenType::RIGHT_BRACE),
                '}' => self.add_token(&mut list, TokenType::LEFT_BRACE),
                ',' => self.add_token(&mut list, TokenType::COMMA),
                ';' => self.add_token(&mut list, TokenType::SEMICOLON),
                '.' => self.add_token(&mut list, TokenType::DOT),
                '-' => self.add_token(&mut list, TokenType::MINUS),
                '+' => self.add_token(&mut list, TokenType::PLUS),
                '*' => self.add_token(&mut list, TokenType::STAR),

                '!' => {
                    if self.matching('=') {
                        self.add_token(&mut list, TokenType::BANG_EQUAL)
                    } else {
                        self.add_token(&mut list, TokenType::BANG)
                    }
                }
                '<' => {
                    if self.matching('=') {
                        self.add_token(&mut list, TokenType::LESS_EQUAL)
                    } else {
                        self.add_token(&mut list, TokenType::LESS)
                    }
                }
                '>' => {
                    if self.matching('=') {
                        self.add_token(&mut list, TokenType::GREATER_EQUAL)
                    } else {
                        self.add_token(&mut list, TokenType::GREATER)
                    }
                }
                '=' => {
                    if self.matching('=') {
                        self.add_token(&mut list, TokenType::EQUAL_EQUAL)
                    } else {
                        self.add_token(&mut list, TokenType::EQUAL)
                    }
                }

                '/' => {
                    if self.matching('/') {
                        while let Some(s) = self.peek_by(0) {
                            if s == '\n' {
                                self.line += 1;
                                self.advance();
                                break;
                            }
                            self.advance();
                        }
                    } else if self.matching('*') {
                        // Challenges 4.
                        let mut closed = false;
                        let start = self.line;
                        while let Some(s) = self.peek_by(0) {
                            if s == '*' {
                                if let Some('/') = self.peek_by(1) {
                                    self.advance_by(2);
                                    closed = true;
                                    break;
                                }
                            } else if s == '\n' {
                                self.line += 1;
                            }
                            print!("{s}");
                            self.advance();
                        }
                        if !closed {
                            self.error(
                                self.line,
                                &format!(
                                    "An unclosed multi-line comment that starts on line {start}."
                                ),
                            );
                        }
                    } else {
                        self.add_token(&mut list, TokenType::SLASH);
                    }
                }

                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,

                '"' => {
                    let token_type = self.scan_string();
                    self.add_token(&mut list, token_type);
                }
                _ => {
                    if a.is_numeric() {
                        let token_type = self.scan_number();
                        self.add_token(&mut list, token_type);
                    } else if a.is_alphabetic() {
                        let token_type = self.scan_identifier();
                        self.add_token(&mut list, token_type);
                    } else {
                        self.error(self.line, "Unexpected character.");
                    }
                }
            }
        }
        list.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            line: self.line,
            literal: LiteralValue::Nil,
        });
        list
    }
}

#[cfg(test)]
mod test {
    use crate::libs::lex::LiteralValue::{Nil, Number, String};
    use crate::libs::lex::TokenType::{EOF, EQUAL, IDENTIFIER, LET, NUMBER, SEMICOLON, STRING};
    use crate::libs::lex::{Token, TokenType};
    use crate::Lox;
    use std::collections::HashMap;

    #[test]
    fn test() {
        let keywords: HashMap<&'static str, TokenType> = HashMap::from([
            ("or", TokenType::OR),
            ("and", TokenType::AND),
            ("box", TokenType::BOX),
            ("else", TokenType::ELSE),
            ("if", TokenType::IF),
            ("fun", TokenType::FUN),
            ("print", TokenType::PRINT),
            ("return", TokenType::RETURN),
            ("super", TokenType::SUPER),
            ("self", TokenType::SELF),
            ("true", TokenType::TRUE),
            ("false", TokenType::FALSE),
            ("let", TokenType::LET),
            ("while", TokenType::WHILE),
            ("nil", TokenType::NIL),
        ]);

        let file = "let x = \"smth\";".to_string();

        let mut lex = Lox {
            file,

            start_pos: 0,
            current_pos: 0,
            gotten_error: false,
            line: 1,

            keywords,
        };
        let a = lex.get_token_list();
        let a_correct = vec![
            Token::new(LET, Nil, 1, "let".to_string()),
            Token::new(IDENTIFIER, Nil, 1, "x".to_string()),
            Token::new(EQUAL, Nil, 1, "=".to_string()),
            Token::new(
                STRING,
                String("smth".to_string()),
                1,
                "\"smth\"".to_string(),
            ),
            Token::new(SEMICOLON, Nil, 1, ";".to_string()),
            Token::new(EOF, Nil, 1, "".to_string()),
        ];
        assert_eq!(a.len(), a_correct.len());
        for i in 0..a.len() {
            assert_eq!(a[i].to_string(), a_correct[i].to_string());
        }
    }
}
