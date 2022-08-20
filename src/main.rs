mod libs;
use libs::lex::Lox;

fn main() {
    env_logger::init();
    let lex = Lox::init();
    if let Err(error) = lex {
        eprintln!("Gotten error: {}", error);
        return;
    }
    let mut lex = lex.unwrap();
    lex.run();
}
