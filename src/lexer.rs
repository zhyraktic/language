#[derive(Debug)]
pub enum Token {
    // literals
    Number(f64),
    Ident(String),
    Str(String),

    // operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,

    // punctuation
    LParen,
    RParen,
    Semicolon,

    // keywords
    Let,

    Eof,
}

pub struct Lexer {
    chars: Vec<char>,
    cpos: usize,
    cline: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            chars: source.chars().collect(),
            cpos: 0,
            cline: 1,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.cpos).copied()
    }

    pub fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.cpos += 1;
        if c == '\n' {
            self.cline += 1;
        }
        Some(c)
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\n' || c == '\t' {
                self.advance();
            } else {
                break;
            }
        }
    }
    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        match self.advance() {
            None => Ok(Token::Eof),
            Some(c) => match c {
                '+' => Ok(Token::Plus),
                _ => Err(format!(
                    "Unexpected character '{}' on line {}",
                    c, self.cline
                )),
            },
        }
    }
}
