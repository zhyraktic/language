mod lexer;

use lexer::Lexer;

fn main() {
    let mut lx = Lexer::new("  + -");
    lx.skip_whitespace();
    println!("{:?}", lx.next_token());
    println!("{:?}", lx.next_token());
}
