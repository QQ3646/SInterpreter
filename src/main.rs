use log::{debug, error, log_enabled, info, Level};

mod libs;
use libs::lex::Lexer;

fn main() {
    env_logger::init();
    let lex = Lexer::init();
    if let Err(error) = lex {
        eprintln!("Gotten error: {}", error);
        return;
    }
    let mut lex = lex.unwrap();
    lex.run();
}
