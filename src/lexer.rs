use std::process;

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(String),
    Symbol(String),
    Comment(String),
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Identifier(ident) => ident.clone(),
            Token::StringLiteral(literal) => literal.clone(),
            Token::NumberLiteral(number) => number.clone(),
            Token::Symbol(symbol) => symbol.clone(),
            Token::Comment(comment) => comment.clone(),
        }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
        }
    }

    pub fn unwrap_next_or_error(&mut self) -> char {
        match self.input.chars().nth(self.position) {
            Some(c) => c,
            None => {
                eprintln!("Unexpected end of input");
                process::exit(1);
            },
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.input = self.input.trim_start();
        if self.position >= self.input.len() {
            return None;
        }

        let c = self.unwrap_next_or_error();
        if c.is_alphabetic() {
            let start = self.position;
            while self.unwrap_next_or_error().is_alphanumeric() {
                self.position += 1;
            }
            let ident = &self.input[start..self.position];
            Some(Token::Identifier(ident.to_string()))
        } else if c.is_digit(10) {
            let start = self.position;
            while self.unwrap_next_or_error().is_digit(10) {
                self.position += 1;
            }
            Some(Token::NumberLiteral(self.input[start..self.position].to_string()))
        } else {
            match c {
                '/' => {
                    self.position += 1;
                    if self.unwrap_next_or_error() == '/' {
                        self.position += 1;
                        let start = self.position;
                        while self.unwrap_next_or_error() != '\n' {
                            self.position += 1;
                        }
                        let comment = format!("//{}", self.input[start..self.position].to_string());
                        Some(Token::Comment(comment))
                    } else {
                        Some(Token::Symbol(c.to_string()))
                    }
                }
                '"' => {
                    self.position += 1;
                    let start = self.position;
                    while self.unwrap_next_or_error() != '"' {
                        self.position += 1;
                    }
                    let literal = format!("\"{}\"", self.input[start..self.position].to_string());
                    self.position += 1;
                    Some(Token::StringLiteral(literal))
                }

                '\'' => {
                    self.position += 1;
                    let start = self.position;
                    while self.unwrap_next_or_error() != '\'' {
                        self.position += 1;
                    }
                    let literal = format!("\'{}\'", self.input[start..self.position].to_string());
                    self.position += 1;
                    Some(Token::StringLiteral(literal))
                }
                '=' => {
                    self.position += 1;
                    let d = self.unwrap_next_or_error();
                    if d == '=' {
                        self.position += 1;
                        if self.unwrap_next_or_error() == '=' {
                            self.position += 1;
                            return Some(Token::Identifier("vibeCheckFr".to_string()))
                        }
                        Some(Token::Identifier("vibeCheck".to_string()))
                    } else if d == '>' {
                        self.position += 1;
                        Some(Token::Identifier("ate".to_string()))
                    } else {
                        Some(Token::Identifier("be".to_string()))
                    }
                }
                '>' | '<' => {
                    self.position += 1;
                    let d = self.unwrap_next_or_error();
                    if d == '=' {
                        self.position += 1;
                        Some(Token::Identifier(format!("{}=", c)))
                    } else {
                        Some(Token::Symbol(c.to_string()))
                    }
                }
                '?' => {
                    self.position += 1;
                    if self.unwrap_next_or_error() == '.' {
                        return Some(Token::Symbol("?".to_string()))
                    }
                    if self.unwrap_next_or_error() == '?' {
                        self.position += 1;
                        return Some(Token::Identifier("sussier".to_string()))
                    }
                    Some(Token::Identifier("sussy".to_string()))
                }
                _ => {
                    self.position += 1;
                    Some(Token::Symbol(c.to_string()))
                }
            }
        }
    }
}