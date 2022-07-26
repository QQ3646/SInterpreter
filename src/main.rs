use std::{
    io::Cursor,
    env};
use std::fs::File;

use log::{debug, error, log_enabled, info, Level};

#[allow(non_camel_case_types)]
enum Token {
    LET_T,
    NAME_T(String),
    TYPE_T(Token, Token),
    NAME_OF_TYPE_T(String),
    ASSIGMENT_T(Token, Token),
    STR_LITERAL_T(String),
    INT_LITERAL_T(i32),
    FUNCTION_T(Vec<Token>),
}

enum State {
    Waiting,

}

struct Lexer {
    state: State,
}

impl Lexer {
    fn new() -> Self {
        Self {
            state: State::Waiting,
        }
    }

    fn put_string(&mut self) {
        match self.state {
            State::Waiting => (),
        }
    }

    fn get_token(&mut self) -> Token {

    }
}

fn main() {
    env_logger::init();

    let file_name = env::args().nth(1).expect("No input file name.");
    let mut file = File::open(&file_name).unwrap();
    let mut cursor = Cursor::new(file);
    println!("File \"{}\" loaded successfully.", file_name);
}
