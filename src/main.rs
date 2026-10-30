mod lexer;

use lexer::{Lexer, Token};

fn main() {
    let mut lx = Lexer::new("3.14 7 2.5 1.2.3");
    loop {
        let tok = lx.next_token();
        println!("{:?}", tok);
        if tok == Ok(Token::Eof) || tok.is_err() {
            break;
        }
    }
}
