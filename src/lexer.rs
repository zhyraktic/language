#[derive(Debug, PartialEq)]
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
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

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

    fn peek(&self) -> Option<char> {
        self.chars.get(self.cpos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.cpos += 1;
        if c == '\n' {
            self.cline += 1;
        }
        Some(c)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\n' || c == '\t' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.cpos + 1).copied()
    }

    fn read_number(&mut self, first: char) -> Result<Token, String> {
        let mut text = String::new();
        text.push(first);

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
                text.push(c);
            } else {
                break;
            }
        }
        if self.peek() == Some('.') && matches!(self.peek_next(), Some('0'..='9')) {
            self.advance();
            text.push('.');
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    self.advance();
                    text.push(c);
                } else {
                    break;
                }
            }
        }
        Ok(Token::Number(text.parse::<f64>().unwrap()))
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        match self.advance() {
            None => Ok(Token::Eof),
            Some(c) => match c {
                '+' => Ok(Token::Plus),
                '-' => Ok(Token::Minus),
                '*' => Ok(Token::Star),
                '/' => Ok(Token::Slash),
                '(' => Ok(Token::LParen),
                ')' => Ok(Token::RParen),
                ';' => Ok(Token::Semicolon),
                '=' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Ok(Token::EqualEqual)
                    }
                    _ => Ok(Token::Equal),
                },
                '!' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Ok(Token::BangEqual)
                    }
                    _ => Ok(Token::Bang),
                },
                '<' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Ok(Token::LessEqual)
                    }
                    _ => Ok(Token::Less),
                },
                '>' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Ok(Token::GreaterEqual)
                    }
                    _ => Ok(Token::Greater),
                },
                c if c.is_ascii_digit() => self.read_number(c),
                _ => Err(format!(
                    "Unexpected character '{}' on line {}",
                    c, self.cline
                )),
            },
        }
    }
}
