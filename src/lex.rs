use std::{env, fs, io};
use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
enum Token {
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
    STRING(String),
    NUMBER(f64),

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
    FIELDS,
    FUNCTION,
    NIL,

    EOF,
}

pub struct Lexer {
    file: String,

    current_pos: usize,
    gotten_error: bool,
    line: usize,

    keywords: HashMap<&'static str, Token>,
}

impl Lexer {
    pub fn init() -> Result<Self, io::Error> {
        let file_name = env::args().nth(1).unwrap_or_else(|| {
            println!("No input file name. \"main.slsf\" will be used instead.");
            //slsf - simple language source file
            "main.slsf".to_string()
        });
        let file = fs::read_to_string(file_name)?;
        let keywords: HashMap<&'static str, Token> = HashMap::from([("or", Token::OR),
            ("and", Token::AND), ("box", Token::BOX), ("else", Token::ELSE), ("if", Token::IF),
            ("fun", Token::FUNCTION), ("print", Token::PRINT), ("return", Token::RETURN),
            ("super", Token::SUPER), ("self", Token::SELF), ("true", Token::TRUE),
            ("false", Token::FALSE), ("let", Token::LET), ("while", Token::WHILE), ("nil", Token::NIL)]);

        Ok(Self {
            file,

            current_pos: 0,
            gotten_error: false,
            line: 1,

            keywords,
        })
    }

    fn error(&mut self, line: usize, message: &str) {
        self.gotten_error = true;
        self.report_error(line, message);
    }

    fn report_error(&mut self, line: usize, message: &str) {
        eprintln!("Error on line {}: {}", line, message);
    }

    pub fn run(&mut self) -> usize {
        let tokens = self.get_token_list();

        println!("Tokens count: {}", tokens.len());
        if self.gotten_error {
            0
        } else {
            // self.build_ast();
            for token in &tokens {
                println!("{:?}", token);
            }
            tokens.len()
        }
    }

    fn scan_identifier(&mut self) -> Token {
        let start = self.current_pos;
        while let Some('0'..='9' | 'a'..='z' | 'A'..='Z' | '_') = self.peek(0) {
            self.advance();
        }

        let text = self.file[start - 1..self.current_pos].to_string();
        let token_option = self.keywords.get(&text as &str);
        (if let None = token_option {
            return Token::IDENTIFIER;
        });
        token_option.unwrap().clone()
    }

    fn scan_string(&mut self) -> Token {
        let start = self.current_pos;
        while let Some(s) = self.peek(0) {
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

        Token::STRING(self.file[start..self.current_pos - 1].to_string())
    }

    fn scan_number(&mut self) -> Token {
        let start = self.current_pos;

        while let Some('0'..='9') = self.peek(0) {
            self.advance();
        }

        if let Some('.') = self.peek(0) {
            if let Some('0'..='9') = self.peek(1) {
                self.advance();

                while let Some('0'..='9') = self.advance() {}
            }
        }

        Token::NUMBER(self.file[start - 1..self.current_pos].parse::<f64>().unwrap_or_else(|_| {
            self.error(self.line, "Failed to parse number.");
            f64::NAN
        }))
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

    fn peek(&self, pos: usize) -> Option<char> {
        self.file.chars().nth(self.current_pos + pos)
    }

    fn advance(&mut self) -> Option<char> {
        self.current_pos += 1;
        self.file.chars().nth(self.current_pos - 1)
    }

    fn add_token(list: &mut Vec<Token>, token_type: Token) {
        list.push(token_type);
    }

    fn get_token_list(&mut self) -> Vec<Token> {
        let mut list: Vec<Token> = Vec::new();
        while let Some(a) = self.advance() {
            match a {
                '(' => Self::add_token(&mut list, Token::LEFT_PAREN),
                ')' => Self::add_token(&mut list, Token::RIGHT_PAREN),
                '{' => Self::add_token(&mut list, Token::RIGHT_BRACE),
                '}' => Self::add_token(&mut list, Token::LEFT_BRACE),
                ',' => Self::add_token(&mut list, Token::COMMA),
                ';' => Self::add_token(&mut list, Token::SEMICOLON),
                '.' => Self::add_token(&mut list, Token::DOT),
                '-' => Self::add_token(&mut list, Token::MINUS),
                '+' => Self::add_token(&mut list, Token::PLUS),
                '*' => Self::add_token(&mut list, Token::STAR),

                '!' => if self.matching('=') {
                    Self::add_token(&mut list, Token::BANG_EQUAL)
                } else {
                    Self::add_token(&mut list, Token::BANG)
                },
                '<' => if self.matching('=') {
                    Self::add_token(&mut list, Token::LESS_EQUAL)
                } else {
                    Self::add_token(&mut list, Token::LESS)
                },
                '>' => if self.matching('=') {
                    Self::add_token(&mut list, Token::GREATER_EQUAL)
                } else {
                    Self::add_token(&mut list, Token::GREATER)
                },
                '=' => if self.matching('=') {
                    Self::add_token(&mut list, Token::EQUAL_EQUAL)
                } else {
                    Self::add_token(&mut list, Token::EQUAL)
                },

                '/' => {
                    if self.matching('/') {
                        while let Some(s) = self.peek(0) {
                            if s == '\n' {
                                break;
                            }
                            self.advance();
                        }
                    } else {
                        Self::add_token(&mut list, Token::SLASH);
                    }
                }

                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,

                '"' => Self::add_token(&mut list, self.scan_string()),
                _ => {
                    if a.is_numeric() {
                        Self::add_token(&mut list, self.scan_number());
                    } else if a.is_alphabetic() {
                        Self::add_token(&mut list, self.scan_identifier());
                    } else {
                        self.error(self.line, "Unexpected character.");
                    }
                }
            }
        }
        list.push(Token::EOF);
        list
    }
}